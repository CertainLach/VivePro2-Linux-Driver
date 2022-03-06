// Generate file similar to openvr.json
// Initally i wrote api generator from it, but then realized driver api isn't defined in it,
// so i wrote this .cpp => openvr.json converter

local a = import './a.json';

local
	merge(a, b) =
		if a == null then b
		else if b == null then a
		else if std.type(a) == 'array' && std.type(b) == 'array' then a + b
		else if std.type(a) == 'object' && std.type(b) == 'object' then {
			[k]: merge(std.get(a, k), std.get(b, k)) for k in std.set(std.objectFields(a) + std.objectFields(b))
		}
		else error "can't merge %v and %v" % [a, b];

local
	makeValue(v) =
		if v.kind == "ImplicitCastExpr" then makeValue(v.inner[0])
		else if v.kind == "ConstantExpr" || v.kind == "IntegerLiteral" then v.value
		else if v.kind == "StringLiteral" then v.value[1:std.length(v.value) - 1]
		else if v.kind == "InitListExpr" then "{%s}" % std.join(", ", std.map(makeValue, v.inner))
		else if v.kind == "DeclRefExpr" then "REF"
		else if v.kind == "CXXNullPtrLiteralExpr" then "NULL"
		else if v.kind == "UnaryOperator" && v.opcode == "-" then "-%s" % makeValue(v.inner[0])
		else if v.kind == "BinaryOperator" && v.opcode == "*" then "%s * %s" % [makeValue(v.inner[0]), makeValue(v.inner[1])]
		else error "" + v,
	makeNamespace(a, ns = '') =
		local
			makeTypedef(def) = {
				typedef: def.name,
				type: def.type.qualType,
			},
			makeParam(param) = {
				paramname: param.name,
				paramtype: param.type.qualType,
			},
			makeMethod(method) = {
				methodname: method.name,
				returntype: std.stripChars(method.type.qualType[:std.findSubstr("(", method.type.qualType)[0]], " "),
				params: std.map(makeParam, std.filter(function(p) p.kind == "ParmVarDecl", std.get(method, "inner", []))),
			},
			makeClass(class) =
				local methods = std.map(makeMethod, std.filter(function(v) v.kind == "CXXMethodDecl" && !std.get(v, "isImplicit", false), std.get(class, "inner", [])));
				std.map(function(m) m {
					classname: class.name,
				}, methods),
			makeField(field) = {
				fieldname: field.name,
				fieldtype: field.type.qualType,
			},
			makeConst(const) = {
				constname: const.name,
				consttype: const.type.qualType,
				constval: makeValue(const.inner[0]),
			},
			makeStruct(struct) = {
				struct: struct.name,
				fields: std.map(makeField, std.filter(function(f) f.kind == "FieldDecl", struct.inner)),
			},
			makeEnumValue(value) = {
				name: value.name,
				value: if !("inner" in value) then null else makeValue(value.inner[0]),
			},
			makeEnum(enum) = {
				enumname: enum.name,
				values: std.map(makeEnumValue, std.filter(function(f) f.kind == "EnumConstantDecl", enum.inner))
			},
			makeUnion(union) = {
				values: std.map(makeField, std.filter(function(v) v.kind == "FieldDecl", union.inner))
			};
		std.foldr(merge, std.map(makeNamespace, std.filter(function(c) c.kind == "NamespaceDecl", a.inner)), {}) + {
			typedefs+: std.map(makeTypedef, std.filter(function(c) c.kind == "TypedefDecl" && !std.get(c, "isImplicit", false) && !std.startsWith(std.get(c.loc, "file", ""), "/nix/") && !("includedFrom" in c.loc), a.inner)),
			methods+: std.join([], std.map(makeClass, std.filter(function(c) c.kind == "CXXRecordDecl" && c.tagUsed == "class", a.inner))),
			consts+: std.map(makeConst, std.filter(function(c) c.kind == "VarDecl", a.inner)),
			structs+: std.map(makeStruct, std.filter(function(c) "name" in c && c.kind == "CXXRecordDecl" && c.tagUsed == "struct" && "inner" in c, a.inner)),
			enums+: std.map(makeEnum, std.filter(function(c) c.kind == "EnumDecl", a.inner)),
			/// In openvr.json there is no definition for EventData_t
			unions+: std.map(makeUnion, std.filter(function(c) c.kind == "CXXRecordDecl" && c.tagUsed == "union", a.inner)),
		};

makeNamespace(a)
