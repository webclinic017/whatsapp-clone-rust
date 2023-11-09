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
		Image string					`json:"image,omitempty"`
		Replicas ReplicasSpec `json:"replicas,omitempty"`
		SecretName string			`json:"secretName,omitempty"`
		Port int32						`json:"port,omitempty"`
	}

	ReplicasSpec struct {
		Min int32 `json:"min,omitempty"`
		Max int32 `json:"max,omitempty"`
	}
)