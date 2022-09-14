(module
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (type $none_=>_none (func))
 (memory $0 0)
 (export "run" (func $assembly/index/run))
 (export "memory" (memory $0))
 (start $~start)
 (func $assembly/index/run (param $0 i32) (param $1 i32) (result i32)
  i32.const 1
  i32.load8_u $0
 )
 (func $~start
  i32.const 1
  memory.grow $0
  drop
  i32.const 0
  i32.const 24
  i32.store8 $0
 )
)
