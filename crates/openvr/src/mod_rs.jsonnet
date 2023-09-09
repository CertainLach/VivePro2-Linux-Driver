local api = import './openvr.json';

local
	groupBy(field, array) =
		local set = std.set(std.map(function(e) e[field], array));
		{
			[key]: std.filter(function(e) e[field] == key, array) for key in set
		};

local
	blacklistConsts = {k_InterfaceVersions: null},
	blacklistFields = {
		TrackedControllerRole_Max: null,
		k_EButton_SteamVR_Touchpad: null, k_EButton_SteamVR_Trigger: null,
		k_EButton_Dashboard_Back: null,
		k_EButton_IndexController_A: null, k_EButton_IndexController_B: null, k_EButton_IndexController_JoyStick: null,
		VRSkeletalTrackingLevel_Count: null, VRSkeletalTrackingLevel_Max: null, VRInputString_All: null,
	},
	fixedTypes = {
		void: "c_void",
		uint64_t: "u64",
		uint32_t: "u32",
		uint16_t: "u16",
		uint8_t: "u8",
		int32_t: "i32",
		int: "i32",
		float: "f32",
		double: "f64",
		char: "c_char",
		"unsigned short": "u16",
		_Bool: "bool",

		'vr::IOBufferHandle_t *': '*const c_void',
		'ID3D12Resource *': '*const c_void',
		'ID3D12CommandQueue *': '*const c_void',

		'uint32_t (*)[2]': '*mut [u32; 2]',
		'const vr::IVRDriverDirectModeComponent::SubmitLayerPerEye_t (&)[2]': '*const c_void',

		'const vr::IVRDriverDirectModeComponent::SwapTextureSetDesc_t *': '*const c_void',
		'vr::IVRDriverDirectModeComponent::SwapTextureSet_t *': '*mut c_void',
	},
	unions = {
		'VREvent_Data_t': 'Union0',
		'VROverlayIntersectionMaskPrimitive_Data_t': 'Union1',
	};

local
	cleanupDefName(name_) =
		local name = std.stripChars(name_, " ");
		if std.startsWith(name, "vr::") then cleanupDefName(name)
		else name,
	fixTypeName_(typ_) =
		local typ = std.stripChars(typ_, " ");
		if std.get(fixedTypes, typ) != null then fixedTypes[typ]
		else if std.startsWith(typ, "vr::") then fixTypeName_(typ[4:])
		else if std.startsWith(typ, "const ") && (std.endsWith(typ, "*") || std.endsWith(typ, "&")) then "*const %s" % fixTypeName_(typ[5:std.length(typ) - 1])
		else if std.endsWith(typ, "*const") then "*const %s" % fixTypeName_(typ[:std.length(typ) - 6])
		else if std.endsWith(typ, "*") then "*mut %s" % fixTypeName_(typ[:std.length(typ) - 1])
		else if std.endsWith(typ, "&") then "*mut %s" % fixTypeName_(typ[:std.length(typ) - 1])
		else if std.endsWith(typ, "]") then local poss = std.findSubstr("[", typ), pos = poss[std.length(poss)-1];
		"[%s; %s]" % [
			fixTypeName_(typ[:pos]),
			typ[pos+1:std.length(typ) - 1],
		]
		else typ,
	fixTypeName(typ_) =
		local typ = fixTypeName_(typ_);
		assert std.type(typ) == "string" : "%s => %s" % [ typ_, typ ];
		if std.startsWith(typ, "*mut I") && !std.startsWith(typ, "*mut Input") then "*const VtableRef<%sVtable>" % typ[5:]
		else typ,

	fixFieldName(field) =
		if field == "type" then "typ" else field;

local
	makeTypeDef(typ) =
		local type = fixTypeName(typ.type);
		"pub type %s = %s;" % [
			cleanupDefName(typ.typedef),
			if std.startsWith(type, "union ") then unions[type[6:]]
			else if type == "&()" then "*const c_void"
			else type,
		],
	makeEnumValue(val) =
		if val.value != null then "\t%s = %s," % [val.name, val.value]
		else "\t%s," % val.name,
	makeEnumDef(enum) =
		("#[derive(Clone, Copy, PartialEq, Eq, Debug)]\n#[repr(i32)]\npub enum %s {\n" % cleanupDefName(enum.enumname)) +
		std.join("\n", std.map(makeEnumValue, std.filter(function(value) std.get(blacklistFields, value.name, false) == false, enum.values))) +
		"\n}",
	makeConst(const) =
		assert const.consttype[0:6] == "const ";
		(if const.constname in blacklistConsts then "// " else "") +
		if const.consttype == "const char *const" || const.consttype == "const char *" then
		"pub const %s: *const c_char = real_c_string!(\"%s\");" % [const.constname, const.constval]
		else
		"pub const %s: %s = %s;" % [const.constname, fixTypeName(const.consttype[6:]), const.constval],
	makeStructField(field) =
		"\tpub %s: %s," % [fixFieldName(field.fieldname), fixTypeName(field.fieldtype)],
	makeStruct(struct) =
		(if struct.struct == "vr::(anonymous)" then "/*" else "") +
		("#[derive(Clone, Copy)]\n#[repr(C)]\npub struct %s {\n" % cleanupDefName(struct.struct)) +
		std.join("\n", std.map(makeStructField, struct.fields)) +
		"\n}" +
		(if struct.struct == "vr::(anonymous)" then "*/" else ""),
	makeParam(param) =
		"%s: %s" % [fixFieldName(param.paramname), fixTypeName(param.paramtype)],
	makeMethod(method) =
		local params = std.get(method, "params", []);
		"\tfn %s(&self%s)%s;" % [
			method.methodname,
			if std.length(params) == 0 then "" else ", " + std.join(", ", std.map(makeParam, method.params)),
			local ret = fixTypeName(method.returntype); if ret == "c_void" then "" else " -> %s" % ret,
		],
	makeClass(name, methods) =
		("#[vtable]\npub trait %s {\n" % cleanupDefName(name)) +
		std.join("\n", std.map(makeMethod, methods)) +
		"\n}",
	makeUnionValue(value) =
		"\t%s: %s," % [fixFieldName(value.fieldname), fixTypeName(value.fieldtype)],
	makeUnion(union) =
		("#[derive(Clone, Copy)]\npub union %s {\n" % cleanupDefName(union.name)) +
		std.join("\n", std.map(makeUnionValue, union.values)) +
		"\n}";

|||
	#![allow(
		non_camel_case_types,
		dead_code,
		non_snake_case,
		non_upper_case_globals,
		clippy::not_unsafe_ptr_arg_deref,
	)]

	use cppvtbl::{vtable, VtableRef};
	use real_c_string::real_c_string;
	use std::ffi::c_void;
	use std::os::raw::c_char;

	// Graphic api stubs
	type VkDevice_T = ();
	type VkInstance_T = ();
	type VkQueue_T = ();
	type VkPhysicalDevice_T = ();

||| +
std.join('\n', std.map(makeUnion, std.makeArray(std.length(api.unions), function(i) api.unions[i] {name: 'Union%d' % i}))) + '\n' +
std.join('\n', std.map(makeTypeDef, api.typedefs)) + '\n' +
std.join('\n', std.map(makeEnumDef, api.enums)) + '\n' +
std.join('\n', std.map(makeConst, api.consts)) + '\n' +
std.join('\n', std.map(makeStruct, api.structs)) + '\n' +
std.join('\n', std.objectValues(std.mapWithKey(makeClass, groupBy('classname', std.filter(function(m) !std.startsWith(m.classname, "C"), api.methods)))))
