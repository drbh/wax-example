; ModuleID = 'artifacts/add.bc'
source_filename = "artifacts/add.bc"
target datalayout = "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v16:16:16-v24:32:32-v32:32:32-v48:64:64-v64:64:64-v96:128:128-v128:128:128-v192:256:256-v256:256:256-v512:512:512-v1024:1024:1024-n8:16:32"
target triple = "air64_v29-apple-macosx27.2.0"

; Function Attrs: convergent nounwind
define void @add(float addrspace(1)* %0, float addrspace(1)* %1, float addrspace(1)* %2, <3 x i32> %3, <3 x i32> %4) #0 {
  %6 = extractelement <3 x i32> %3, i32 0
  %7 = mul i32 %6, 1
  %8 = mul i32 0, 256
  %9 = add i32 %7, %8
  %10 = mul i32 0, 256
  %11 = add i32 %9, %10
  %12 = getelementptr float, float addrspace(1)* %1, i32 %11
  %13 = load float, float addrspace(1)* %12, align 4
  %14 = extractelement <3 x i32> %3, i32 0
  %15 = mul i32 %14, 1
  %16 = mul i32 0, 256
  %17 = add i32 %15, %16
  %18 = mul i32 0, 256
  %19 = add i32 %17, %18
  %20 = getelementptr float, float addrspace(1)* %2, i32 %19
  %21 = load float, float addrspace(1)* %20, align 4
  %22 = fadd float %13, %21
  %23 = extractelement <3 x i32> %4, i32 0
  %24 = extractelement <3 x i32> %4, i32 1
  %25 = extractelement <3 x i32> %4, i32 2
  %26 = extractelement <3 x i32> %3, i32 0
  %27 = mul i32 %26, 1
  %28 = mul i32 %23, 256
  %29 = add i32 %27, %28
  %30 = mul i32 %24, 256
  %31 = add i32 %29, %30
  %32 = getelementptr float, float addrspace(1)* %0, i32 %31
  store float %22, float addrspace(1)* %32, align 4
  ret void
}

attributes #0 = { convergent nounwind }

!air.kernel = !{!0}
!llvm.module.flags = !{!8, !9, !10, !11, !12, !13, !14, !15}
!air.version = !{!16}
!air.language_version = !{!17}
!llvm.ident = !{!18}

!0 = !{void (float addrspace(1)*, float addrspace(1)*, float addrspace(1)*, <3 x i32>, <3 x i32>)* @add, !1, !2}
!1 = !{}
!2 = !{!3, !4, !5, !6, !7}
!3 = !{i32 0, !"air.buffer", !"air.location_index", i32 0, i32 1, !"air.read_write", !"air.address_space", i32 1, !"air.arg_type_size", i32 4, !"air.arg_type_align_size", i32 4, !"air.arg_type_name", !"float", !"air.arg_name", !"param0"}
!4 = !{i32 1, !"air.buffer", !"air.location_index", i32 1, i32 1, !"air.read_write", !"air.address_space", i32 1, !"air.arg_type_size", i32 4, !"air.arg_type_align_size", i32 4, !"air.arg_type_name", !"float", !"air.arg_name", !"param1"}
!5 = !{i32 2, !"air.buffer", !"air.location_index", i32 2, i32 1, !"air.read_write", !"air.address_space", i32 1, !"air.arg_type_size", i32 4, !"air.arg_type_align_size", i32 4, !"air.arg_type_name", !"float", !"air.arg_name", !"param2"}
!6 = !{i32 3, !"air.thread_position_in_threadgroup", !"air.arg_type_name", !"uint3", !"air.arg_name", !"_tp"}
!7 = !{i32 4, !"air.threadgroup_position_in_grid", !"air.arg_type_name", !"uint3", !"air.arg_name", !"_tg"}
!8 = !{i32 1, !"wchar_size", i32 4}
!9 = !{i32 7, !"frame-pointer", i32 2}
!10 = !{i32 7, !"air.max_device_buffers", i32 31}
!11 = !{i32 7, !"air.max_constant_buffers", i32 31}
!12 = !{i32 7, !"air.max_threadgroup_buffers", i32 31}
!13 = !{i32 7, !"air.max_textures", i32 128}
!14 = !{i32 7, !"air.max_read_write_textures", i32 8}
!15 = !{i32 7, !"air.max_samplers", i32 16}
!16 = !{i32 2, i32 8, i32 0}
!17 = !{!"Metal", i32 4, i32 0, i32 0}
!18 = !{!"wax-metal"}
