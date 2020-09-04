

fn main() {
    
}

//deno_core
// fn new_isolate() {
//     let startup_data = StartupData::Script(Script {
//         source: include_str!("foo.ts"),
//         filename: "foo.ts"
//     });

//     let mut isolate = CoreIsolate::new(startup_data, false);

//     isolate
// }

//rusty_v8
// fn rusty() {
//     let platform = v8::new_default_platform().unwrap();
//     v8::V8::initialize_platform(platform);
//     v8::V8::initialize();

//     let isolate = &mut v8::Isolate::new(Default::default());

//     let scope = &mut v8::HandleScope::new(isolate);
//     let context = v8::Context::new(scope);
//     let scope = &mut v8::ContextScope::new(scope, context);

//     let global = context.global(scope);

//     let value = v8::String::new(scope, "test").unwrap().into();
//     let name = v8::String::new(scope, "name").unwrap().into();

//     global.set(scope, name, value);

//     let code = v8::String::new(scope, "'Hello ' + name").unwrap();
//     println!("javascript code: {}", code.to_rust_string_lossy(scope));

//     let resource_name = v8::String::new(scope, "foo.js").unwrap();
//     let resource_line_offset = v8::Integer::new(scope, 4);
//     let resource_column_offset = v8::Integer::new(scope, 5);
//     let resource_is_shared_cross_origin = v8::Boolean::new(scope, true);
//     let script_id = v8::Integer::new(scope, 123);
//     let source_map_url = v8::String::new(scope, "source_map_url").unwrap();
//     let resource_is_opaque = v8::Boolean::new(scope, true);
//     let is_wasm = v8::Boolean::new(scope, false);
//     let is_module = v8::Boolean::new(scope, false);

//     let script_origin = v8::ScriptOrigin::new(
//       resource_name.into(),
//       resource_line_offset,
//       resource_column_offset,
//       resource_is_shared_cross_origin,
//       script_id,
//       source_map_url.into(),
//       resource_is_opaque,
//       is_wasm,
//       is_module,
//     );

//     let script = v8::Script::compile(scope, code, Some(&script_origin)).unwrap();
//     let result = script.run(scope).unwrap();
//     let result = result.to_string(scope).unwrap();
//     println!("result: {}", result.to_rust_string_lossy(scope));

//     println!("End!");
// }
