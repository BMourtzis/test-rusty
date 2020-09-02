use rusty_v8 as v8;

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let global = context.global(scope);

    let value = v8::String::new(scope, "test").unwrap().into();
    let name = v8::String::new(scope, "name").unwrap().into();

    global.set(scope, name, value);

    let code = v8::String::new(scope, "'Hello ' + name").unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("result: {}", result.to_rust_string_lossy(scope));

    println!("End!");
}
