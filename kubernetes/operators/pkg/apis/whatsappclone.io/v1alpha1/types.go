package v1alpha1

import metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"

type (
	// +genclient
	// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
	Application struct {
		metav1.TypeMeta 		 `json:",inline"`
		metav1.ObjectMeta 	 `json:"metadata,omitempty"`

		Spec ApplicationSpec `json:"spec,omitempty"`
	}

	// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
	ApplicationList struct {
		metav1.TypeMeta 		`json:",inline"`
		metav1.ListMeta 		`json:"metadata,omitempty"`

		Items []Application `json:"items,omitempty"`
	}

	ApplicationSpec struct {
		Pods PodsSpec 		`json:"pods,omitempty"`
		SecretName string `json:"secretName,omitempty"`
		Ports []uint 			`json:"ports,omitempty"`
	}

	PodsSpec struct {
		Min uint `json:"min,omitempty"`
		Max uint `json:"max,omitempty"`
	}
)