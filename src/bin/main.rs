use core::{ffi::CStr, mem::MaybeUninit};

use slang::{IBlob, IComponentType, IGlobalSession, IModule, ISession, IUnknown};

fn main() {
    let g_session = slang::create_global_session(&slang::GlobalSessionDesc::default())
        .expect("create_global_session failed");
    let targets = [slang::TargetDesc {
        format: slang::ffi::SLANG_SPIRV,
        profile: g_session.find_profile(c"glsl_450"),
        ..Default::default()
    }];
    let session = g_session
        .create_session(&slang::SessionDesc {
            targets: targets.as_ptr(),
            target_count: targets.len() as _,
            ..Default::default()
        })
        .unwrap();
    let mut diag = MaybeUninit::zeroed();
    let module = session.load_module(c"test", Some(&mut diag));
    let diag = unsafe { diag.assume_init() };
    if let Some(d) = diag {
        let diag_str = unsafe {
            CStr::from_ptr(d.get_buffer_pointer() as _)
                .to_str()
                .unwrap_unchecked()
        };
        for x in diag_str.lines() {
            eprintln!("diag: {x}");
        }
    }
    let module = module.expect("Failed to load module");

    let mut program_components =
        Vec::with_capacity(1 + module.get_defined_entry_point_count() as usize);
    program_components.push(
        module
            .clone_cast::<slang::IComponentTypePtr>()
            .expect("failed to cast"),
    );
    for n in 0..module.get_defined_entry_point_count() {
        program_components.push(
            module
                .get_defined_entry_point(n)
                .expect("failed to get entry point")
                .clone_cast()
                .expect("failed to cast"),
        );
    }
    let mut diag = MaybeUninit::zeroed();
    let program = session.create_composite_component_type(&program_components, Some(&mut diag));
    if let Some(d) = unsafe { diag.assume_init() } {
        println!("diag: {:?}", unsafe {
            CStr::from_ptr(d.get_buffer_pointer() as _)
        });
    }
    let program = program.expect("failed to create program");

    let mut diag = MaybeUninit::zeroed();
    let layout = program.get_layout(0, Some(&mut diag));
    if let Some(d) = unsafe { diag.assume_init() } {
        println!("diag: {:?}", unsafe {
            CStr::from_ptr(d.get_buffer_pointer() as _)
        });
    }
    for n in 0..layout.entry_point_count() {
        let ep = layout.entry_point(n).expect("no entry point?");
        let fr = ep.function();
        let name = fr.name();
        let stage = ep.stage();

        println!("ep {n} {name:?} {stage}");

        for np in 0..ep.parameter_count() {
            let param = ep.parameter(np).expect("no parameter?");
            let pv = param.variable();
            let param_semantic_name = param.semantic_name();
            let param_type = pv.r#type();
            let param_type_name_blob = param_type
                .full_name()
                .expect("spReflectionType_GetFullName failed");
            let param_type_name =
                unsafe { CStr::from_ptr(param_type_name_blob.get_buffer_pointer() as _) };

            println!("  param {np} {param_type_name:?} {param_semantic_name:?}");
        }

        let rt = fr.result_type();
        let rt_name_blob = rt.full_name().expect("spReflectionType_GetFullName failed");
        let rt_name = unsafe { CStr::from_ptr(rt_name_blob.get_buffer_pointer() as _) };
        let rtl = ep.result_var_layout();
        let rt_semantic_name = rtl.semantic_name();

        println!("  rt {rt_name:?} {rt_semantic_name:?}");
    }
    for n in 0..layout.parameter_count() {
        let param = layout.parameter(n).expect("no parameter?");
        let pv = param.variable();
        let param_semantic_name = param.semantic_name();
        let param_type = pv.r#type();
        let param_type_name_blob = param_type.full_name().expect("param_type.full_name failed");
        let param_type_name =
            unsafe { CStr::from_ptr(param_type_name_blob.get_buffer_pointer() as _) };
        let binding_index = param.binding_index();
        let binding_space = param.binding_space();

        println!(
            "param {n} {param_type_name:?} {param_semantic_name:?} {binding_index} {binding_space}"
        );
    }
    let gcb_binding = layout.global_constant_buffer_binding();
    let gcb_size = layout.global_constant_buffer_size();
    println!("global constant buffer: {gcb_binding} {gcb_size}");

    let mut diag = MaybeUninit::zeroed();
    let linked = program.link(Some(&mut diag));
    if let Some(d) = unsafe { diag.assume_init() } {
        println!("diag: {:?}", unsafe {
            CStr::from_ptr(d.get_buffer_pointer() as _)
        });
    }
    let linked = linked.expect("Failed to link");

    let mut diag = MaybeUninit::zeroed();
    let spv_code = linked.get_target_code(0, Some(&mut diag));
    if let Some(d) = unsafe { diag.assume_init() } {
        println!("diag: {:?}", unsafe {
            CStr::from_ptr(d.get_buffer_pointer() as _)
        });
    }
    let spv_code = spv_code.expect("Failed to generate spirv");
    println!("code length: {}", spv_code.get_buffer_size());
    std::fs::write("test.spv", unsafe {
        core::slice::from_raw_parts(
            spv_code.get_buffer_pointer() as *const u8,
            spv_code.get_buffer_size(),
        )
    })
    .expect("Failed to write spv");
}
