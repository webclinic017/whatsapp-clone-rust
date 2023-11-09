package controller

import (
	"context"
	"fmt"
	"time"

	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"

	"github.com/Archisman-Mridha/whatsapp-clone/kubernetes/operators/application/pkg/apis/whatsappclone.io/v1alpha1"
	clientset "github.com/Archisman-Mridha/whatsapp-clone/kubernetes/operators/application/pkg/generated/clientset/versioned"
	informer "github.com/Archisman-Mridha/whatsapp-clone/kubernetes/operators/application/pkg/generated/informers/externalversions/whatsappclone.io/v1alpha1"
	lister "github.com/Archisman-Mridha/whatsapp-clone/kubernetes/operators/application/pkg/generated/listers/whatsappclone.io/v1alpha1"
	"github.com/charmbracelet/log"
	appsV1 "k8s.io/api/apps/v1"
	autoscalingV2 "k8s.io/api/autoscaling/v2"
	coreV1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/api/errors"
	"k8s.io/apimachinery/pkg/util/intstr"
	"k8s.io/apimachinery/pkg/util/runtime"
	"k8s.io/apimachinery/pkg/util/wait"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/tools/cache"
	"k8s.io/client-go/util/workqueue"
)

type Controller struct {
	name string

	// kubeclient is the Kubernetes API server client.
	kubeclient *kubernetes.Clientset

	/*
		NOTE

		1. Clientset abstracts the low-level HTTP communication with the API server for CRUD operations
		regarding the Custom Resource.

		2. Listers are utility components used to cache and index Kubernetes resources, making it faster
		and more efficient to retrieve and filter resources from the cluster. Listers maintain an
		up-to-date local cache of the desired resources and provide methods for querying and filtering
		those resources. This helps reduce the load on the Kubernetes API server.

		3. Informers are built on top of listers and are responsible for watching changes to resources
		in the cluster. They continuously synchronize the local cache with the cluster state.
	*/
	clientset clientset.Interface
	applicationLister lister.ApplicationLister
	// informerSynced is a function that can be used to determine if the informer has synced the
	// lister cache.
	informerSynced cache.InformerSynced

	// The controller watches whatsappclone.io/Application type objects in the Kubernetes cluster.
	// When an event, such as resource creation, update, or deletion, occurs regarding the object, the
	// controller generates an event or task. Those events are enqueued into this work-queue. The
	// work-queue processes events in a controlled and rate-limited manner.
	workQueue workqueue.RateLimitingInterface
}

// NewController returns a new instance of the Controller.
func NewController(kubeclient *kubernetes.Clientset, clientset clientset.Interface, informer informer.ApplicationInformer) *Controller {
	controller := &Controller{
		name: "Application Controller",

		kubeclient: kubeclient,

		clientset: clientset,
		applicationLister: informer.Lister( ),
		informerSynced: informer.Informer( ).HasSynced,

		workQueue: workqueue.NewNamedRateLimitingQueue(workqueue.DefaultControllerRateLimiter( ), "application"),
	}

	log.Info("Setting up event handlers in informer factory")
	informer.Informer( ).AddEventHandler(cache.ResourceEventHandlerFuncs{
		AddFunc: controller.handleObjectAdded,
		UpdateFunc: controller.handleObjectUpdated,
		DeleteFunc: controller.handleObjectDeleted,
	})

	return controller
}

// handleObjectAdded is invoked when an Application object is created in the cluster. It takes the
// Application object, constructs its 'namespace/name' key and puts the key into the work queue.
func(c *Controller) handleObjectAdded(obj interface{}) {
	key, err := cache.MetaNamespaceKeyFunc(obj)
	if err != nil {
		runtime.HandleError(err)
		return
	}

	c.workQueue.Add(key)
}

func(c *Controller) handleObjectUpdated(oldObj interface{}, newObj interface{}) { }

// handleObjectDeleted is invoked when an Application object is deleted from the cluster. It adds
// the object to the work-queue.
func(c *Controller) handleObjectDeleted(obj interface{}) {
	c.workQueue.Add(obj)
}

// Run will set up the event handlers for types we are interested in, as well as syncing informer
// caches and starting workers. It will block until stopCh (in the informerFactory) is closed, at
// which point it will shutdown the workqueue and wait for workers to finish processing their
// current work items.
func (c *Controller) Run(ctx context.Context) error {
	log.Info("Starting Application controller")

	defer runtime.HandleCrash( )
	defer c.workQueue.ShutDown( )

	// Start the informer factory to begin populating the informer cache. Wait for the cache to be
	// synced before starting any workers.
	log.Info("Waiting for informer caches to sync")
	if ok := cache.WaitForCacheSync(ctx.Done( ), c.informerSynced); !ok {
		return fmt.Errorf("failed waiting for informer cache to sync")}

	// Launch a worker to process Application resources.
	log.Info("Starting worker to process Application resources")
	go wait.UntilWithContext(ctx, c.processWorkQueueItems, time.Second)
	log.Info("Worker started")

	<-ctx.Done( )
	log.Info("Shutting down worker")
	return nil
}

// processWorkQueueItems is a long-running method that will continually call the
// processNextWorkQueueItem function in order to read and process an event int the work-queue.
func (c *Controller) processWorkQueueItems(ctx context.Context) {
	for c.processNextWorkQueueItem(ctx) { }
}

// processNextWorkQueueItem will read a single work item off the work-queue and attempt to process
// it, by calling the syncHandler method.
func (c *Controller) processNextWorkQueueItem(ctx context.Context) bool {
	obj, shutdown := c.workQueue.Get( )
	if shutdown {
		return false
	}

	err := func(obj interface{}) error {
		// We call Done here so the workqueue knows we have finished processing this item. We also must
		// remember to call Forget if we do not want this work item being re-queued. For example, we do
		// not call Forget if a transient error occurs, instead the item is put back on the workqueue
		// and attempted again after a back-off period.
		defer c.workQueue.Done(obj)

		key, ok := obj.(string)
		if !ok {
			c.workQueue.Forget(obj)
			runtime.HandleError(fmt.Errorf("expected string in workqueue but got %#v", obj))
			return nil
		}

		if err := c.syncHandler(ctx, key); err != nil {
			// Put the item back in the work-queue to handle any transient errors.
			c.workQueue.AddRateLimited(key)
			return fmt.Errorf("error syncing '%s': %s, requeuing", key, err.Error( ))
		}

		c.workQueue.Forget(obj)
		log.Infof("Successfully synced resource %s", key)
		return nil
	}(obj)

	if err != nil {
		runtime.HandleError(err)}

	return true
}

// syncHandler compares the actual state with the desired (for an Application resource), and
// attempts to converge the two.
func (c *Controller) syncHandler(ctx context.Context, key string) error {
	logger := log.With("resourceName", key)

	namespace, name, err := cache.SplitMetaNamespaceKey(key)
	if err != nil {
		runtime.HandleError(fmt.Errorf("invalid resource key: %s", key))
		return nil
	}

	application, err := c.applicationLister.Applications(namespace).Get(name)
	if err != nil {
		// The Application resource has been deleted.
		if errors.IsNotFound(err) {
			c.kubeclient.AppsV1( ).Deployments(namespace).Delete(ctx, name, metav1.DeleteOptions{ })
			c.kubeclient.AutoscalingV2( ).HorizontalPodAutoscalers(namespace).Delete(ctx, name, metav1.DeleteOptions{ })
			c.kubeclient.CoreV1( ).Services(namespace).Delete(ctx, name, metav1.DeleteOptions{ })

			return nil
		}

		return err
	}

	/* Create Kubernetes Deployment, HorizontalPodAutoscaler and Secret for the Application */ {
		if err := c.createDeployment(ctx, application); err != nil {
			logger.Error("Error creating Kubernetes Deployment : %v", err)
			return err
		}
		logger.Info("Successfully created Kubernetes Deployment")

		if err := c.createHorizontalPodAutoscaler(ctx, application); err != nil {
			logger.Error("Error creating Kubernetes Horizontal Pod Autoscaler : %v", err)
			return err
		}
		logger.Info("Successfully created Kubernetes Horizontal Pod Autoscaler")

		if err := c.createService(ctx, application); err != nil {
			logger.Error("Error creating Kubernetes Service : %v", err)
			return err
		}
		logger.Info("Successfully created Kubernetes Service")
	}

	return nil
}

// createDeployment creates the Kubernetes Deployment for an Application resource.
func(c *Controller) createDeployment(ctx context.Context, application *v1alpha1.Application) error {
	name := application.Name
	namespace := application.Namespace

	podTemplateSpecObject := coreV1.PodTemplateSpec{
		ObjectMeta: metav1.ObjectMeta{
			Labels: map[string]string{
				"microservice": name,
			},
		},

		Spec: coreV1.PodSpec{
			Containers: []coreV1.Container{
				{
					Name: name,

					Image: application.Spec.Image,
					ImagePullPolicy: coreV1.PullIfNotPresent,

					EnvFrom: []coreV1.EnvFromSource{
						{
							SecretRef: &coreV1.SecretEnvSource{
								LocalObjectReference: coreV1.LocalObjectReference{
									Name: application.Spec.SecretName,
								},
							},
						},
					},

					Ports: []coreV1.ContainerPort{
						{
							Name: "gRPC server",
							ContainerPort: application.Spec.Port,
							Protocol: "TCP",
						},
					},

					// TODO: Configure Pod resource requests and limits.
				},
			},
		},
	}

	deploymentObject := &appsV1.Deployment{

		ObjectMeta: metav1.ObjectMeta{
			Name: name,
			Namespace: namespace,
			Labels: map[string]string{
				"app.kubernetes.io/part-of": name,
				"app.kubernetes.io/managed-by": c.name,
			},
		},

		Spec: appsV1.DeploymentSpec{
			Replicas: &application.Spec.Replicas.Min,

			// TODO: Determine Deployment strategy.

			Selector: &metav1.LabelSelector{
				MatchLabels: map[string]string{
					"microservice": name,
				},
			},

			Template: podTemplateSpecObject,
		},
	}

	_, err := c.kubeclient.AppsV1( ).Deployments(namespace).Create(ctx, deploymentObject, metav1.CreateOptions{ })
	return err
}

// createHorizontalPodAutoscaler creates the Kubernetes Horizontal Pod Autoscaler for an Application
// resource.
func(c *Controller) createHorizontalPodAutoscaler(ctx context.Context, application *v1alpha1.Application) error {
	name := application.Name
	namespace := application.Namespace

	var averageCpuUtilization int32= 60

	horizontalPodAutoscalerObject := &autoscalingV2.HorizontalPodAutoscaler{

		ObjectMeta: metav1.ObjectMeta{
			Name: name,
			Namespace: namespace,
			Labels: map[string]string{
				"app.kubernetes.io/part-of": name,
				"app.kubernetes.io/managed-by": c.name,
			},
		},

		Spec: autoscalingV2.HorizontalPodAutoscalerSpec{

			ScaleTargetRef: autoscalingV2.CrossVersionObjectReference{
				APIVersion: "apps/v1",
				Kind: "Deployments",
				Name: name,
			},

			MinReplicas: &application.Spec.Replicas.Min,
			MaxReplicas: application.Spec.Replicas.Max,

			Metrics: []autoscalingV2.MetricSpec{
				{
					Type: autoscalingV2.ContainerResourceMetricSourceType,
					Resource: &autoscalingV2.ResourceMetricSource{
						Name: "cpu",
						Target: autoscalingV2.MetricTarget{
							Type: autoscalingV2.UtilizationMetricType,
							AverageUtilization: &averageCpuUtilization,
						},
					},
				},
			},
		},
	}

	_, err := c.kubeclient.AutoscalingV2( ).HorizontalPodAutoscalers(namespace).Create(ctx, horizontalPodAutoscalerObject, metav1.CreateOptions{ })
	return err
}

// createService creates the Kubernetes Service for an Application resource.
func(c *Controller) createService(ctx context.Context, application *v1alpha1.Application) error {
	name := application.Name
	namespace := application.Namespace

	serviceObject := &coreV1.Service{

		ObjectMeta: metav1.ObjectMeta{
			Name: name,
			Namespace: namespace,
			Labels: map[string]string{
				"app.kubernetes.io/part-of": name,
				"app.kubernetes.io/managed-by": c.name,
			},
		},

		Spec: coreV1.ServiceSpec{
			Selector: map[string]string{
				"microservice": name,
			},

			Ports: []coreV1.ServicePort{
				{
					Name: "gRPC server",
					Port: application.Spec.Port,
					TargetPort: intstr.FromInt32(application.Spec.Port),
				},
			},
		},
	}

	_, err := c.kubeclient.CoreV1( ).Services(namespace).Create(ctx, serviceObject, metav1.CreateOptions{ })
	return err
}