(module
 (type $none_=>_none (func))
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (global $assembly/index/index i32 (i32.const 0))
 (global $assembly/index/value i32 (i32.const 24))
 (global $~lib/memory/__data_end i32 (i32.const 8))
 (global $~lib/memory/__stack_pointer (mut i32) (i32.const 16392))
 (global $~lib/memory/__heap_base i32 (i32.const 16392))
 (memory $0 0)
 (table $0 1 1 funcref)
 (elem $0 (i32.const 1))
 (export "run" (func $assembly/index/run))
 (export "memory" (memory $0))
 (start $~start)
 (func $start:assembly/index
  i32.const 1
  memory.grow $0
  drop
  global.get $assembly/index/index
  global.get $assembly/index/value
  i32.store8 $0
 )
 (func $assembly/index/run (param $a i32) (param $b i32) (result i32)
  (local $var$2 i32)
  i32.const 1
  i32.load8_u $0
  local.set $var$2
  local.get $var$2
 )
 (func $~start
  call $start:assembly/index
 )
)
