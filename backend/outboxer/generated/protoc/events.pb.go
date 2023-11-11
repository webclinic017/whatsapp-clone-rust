// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        v4.23.4
// source: events.proto

package protoc_generated

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// CloudEvents is a specification for describing event data in common formats to provide
// interoperability across services, platforms and systems.
// Read more here - https://github.com/cloudevents/spec.
type CloudEvent struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Required attributes.
	Id          string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	Source      string `protobuf:"bytes,2,opt,name=source,proto3" json:"source,omitempty"`
	SpecVersion string `protobuf:"bytes,3,opt,name=spec_version,json=specVersion,proto3" json:"spec_version,omitempty"`
	Type        string `protobuf:"bytes,4,opt,name=type,proto3" json:"type,omitempty"`
	// Event payload.
	//
	// Types that are assignable to Payload:
	//
	//	*CloudEvent_UserRegisteredEvent
	Payload isCloudEvent_Payload `protobuf_oneof:"payload"`
}

func (x *CloudEvent) Reset() {
	*x = CloudEvent{}
	if protoimpl.UnsafeEnabled {
		mi := &file_events_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CloudEvent) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CloudEvent) ProtoMessage() {}

func (x *CloudEvent) ProtoReflect() protoreflect.Message {
	mi := &file_events_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CloudEvent.ProtoReflect.Descriptor instead.
func (*CloudEvent) Descriptor() ([]byte, []int) {
	return file_events_proto_rawDescGZIP(), []int{0}
}

func (x *CloudEvent) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *CloudEvent) GetSource() string {
	if x != nil {
		return x.Source
	}
	return ""
}

func (x *CloudEvent) GetSpecVersion() string {
	if x != nil {
		return x.SpecVersion
	}
	return ""
}

func (x *CloudEvent) GetType() string {
	if x != nil {
		return x.Type
	}
	return ""
}

func (m *CloudEvent) GetPayload() isCloudEvent_Payload {
	if m != nil {
		return m.Payload
	}
	return nil
}

func (x *CloudEvent) GetUserRegisteredEvent() *UserRegisteredEvent {
	if x, ok := x.GetPayload().(*CloudEvent_UserRegisteredEvent); ok {
		return x.UserRegisteredEvent
	}
	return nil
}

type isCloudEvent_Payload interface {
	isCloudEvent_Payload()
}

type CloudEvent_UserRegisteredEvent struct {
	UserRegisteredEvent *UserRegisteredEvent `protobuf:"bytes,5,opt,name=user_registered_event,json=userRegisteredEvent,proto3,oneof"`
}

func (*CloudEvent_UserRegisteredEvent) isCloudEvent_Payload() {}

type UserRegisteredEvent struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	UserId   string `protobuf:"bytes,1,opt,name=user_id,json=userId,proto3" json:"user_id,omitempty"`
	Name     string `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	Username string `protobuf:"bytes,3,opt,name=username,proto3" json:"username,omitempty"`
}

func (x *UserRegisteredEvent) Reset() {
	*x = UserRegisteredEvent{}
	if protoimpl.UnsafeEnabled {
		mi := &file_events_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UserRegisteredEvent) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UserRegisteredEvent) ProtoMessage() {}

func (x *UserRegisteredEvent) ProtoReflect() protoreflect.Message {
	mi := &file_events_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UserRegisteredEvent.ProtoReflect.Descriptor instead.
func (*UserRegisteredEvent) Descriptor() ([]byte, []int) {
	return file_events_proto_rawDescGZIP(), []int{1}
}

func (x *UserRegisteredEvent) GetUserId() string {
	if x != nil {
		return x.UserId
	}
	return ""
}

func (x *UserRegisteredEvent) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *UserRegisteredEvent) GetUsername() string {
	if x != nil {
		return x.Username
	}
	return ""
}

var File_events_proto protoreflect.FileDescriptor

var file_events_proto_rawDesc = []byte{
	0x0a, 0x0c, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06,
	0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x22, 0xc9, 0x01, 0x0a, 0x0a, 0x43, 0x6c, 0x6f, 0x75, 0x64,
	0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x21, 0x0a,
	0x0c, 0x73, 0x70, 0x65, 0x63, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x0b, 0x73, 0x70, 0x65, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
	0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
	0x74, 0x79, 0x70, 0x65, 0x12, 0x51, 0x0a, 0x15, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x67,
	0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x18, 0x05, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x55, 0x73, 0x65,
	0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74,
	0x48, 0x00, 0x52, 0x13, 0x75, 0x73, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72,
	0x65, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x42, 0x09, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f,
	0x61, 0x64, 0x22, 0x5e, 0x0a, 0x13, 0x55, 0x73, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74,
	0x65, 0x72, 0x65, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x75, 0x73, 0x65,
	0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x75, 0x73, 0x65, 0x72,
	0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61,
	0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61,
	0x6d, 0x65, 0x42, 0x13, 0x5a, 0x11, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x5f, 0x67, 0x65, 0x6e,
	0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x2f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_events_proto_rawDescOnce sync.Once
	file_events_proto_rawDescData = file_events_proto_rawDesc
)

func file_events_proto_rawDescGZIP() []byte {
	file_events_proto_rawDescOnce.Do(func() {
		file_events_proto_rawDescData = protoimpl.X.CompressGZIP(file_events_proto_rawDescData)
	})
	return file_events_proto_rawDescData
}

var file_events_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_events_proto_goTypes = []interface{}{
	(*CloudEvent)(nil),          // 0: events.CloudEvent
	(*UserRegisteredEvent)(nil), // 1: events.UserRegisteredEvent
}
var file_events_proto_depIdxs = []int32{
	1, // 0: events.CloudEvent.user_registered_event:type_name -> events.UserRegisteredEvent
	1, // [1:1] is the sub-list for method output_type
	1, // [1:1] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_events_proto_init() }
func file_events_proto_init() {
	if File_events_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_events_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CloudEvent); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_events_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UserRegisteredEvent); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_events_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*CloudEvent_UserRegisteredEvent)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_events_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_events_proto_goTypes,
		DependencyIndexes: file_events_proto_depIdxs,
		MessageInfos:      file_events_proto_msgTypes,
	}.Build()
	File_events_proto = out.File
	file_events_proto_rawDesc = nil
	file_events_proto_goTypes = nil
	file_events_proto_depIdxs = nil
}
