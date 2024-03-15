Loading from address : BoolectorExpr(#x201fffc8)
Loading from address : 538968008
Loaded value Ok(BoolectorExpr(#x00000001)) from memory
instr : (16, AddSPImmediate(AddSPImmediate { s: Some(false), rd: None, imm: 4 }))
instr : (16, Bx(Bx { rm: LR }))
instr : (32, LdrbImmediate(LdrbImmediate { w: Some(false), add: Some(false), index: true, rt: R0, rn: R7, imm: Some(19) }))
Loading from address : BoolectorExpr(#x201fffcd)
Loading from address : 538968013
Loaded value Ok(BoolectorExpr(#x01)) from memory
instr : (16, CmpImmediate(CmpImmediate { rn: R0, imm: 3 }))
instr : (16, B(B { condition: Hi, imm: 82 }))
instr : (32, Tb(Tb { is_tbh: Some(false), rn: PC, rm: R0 }))
Loading from address : BoolectorExpr(#x2001007f)
Loading from address : 536936575
Loaded value Ok(BoolectorExpr(#x20)) from memory
instr : (32, LdrhImmediate(LdrhImmediate { w: Some(false), add: Some(false), index: Some(true), rt: R1, rn: R7, imm: 10 }))
Loading from address : BoolectorExpr(#x201fffd6)
Loading from address : 538968022
Loaded value Ok(BoolectorExpr((concat (select memory #x201fffd7) (select memory #x201fffd6)))) from memory
instr : (16, MovImmediate(MovImmediate { s: Some(true), rd: R0, imm: 1, carry: None }))
instr : (16, B(B { condition: None, imm: 30 }))
instr : (16, StrhImmediate(StrhImmediate { index: true, add: true, w: false, rt: R5, rn: R4, imm: Some(2) }))
Setting 538968044 to BoolectorExpr((bvand BTOR_2@R5 #x0000ffff))
instr : (16, StrhImmediate(StrhImmediate { index: true, add: true, w: false, rt: R0, rn: R4, imm: Some(0) }))
Setting 538968042 to BoolectorExpr(#x00000001)
instr : (16, AddSPImmediate(AddSPImmediate { s: Some(false), rd: None, imm: 16 }))
instr : (16, Pop(Pop { registers: RegisterList { regs: [R4, R5, R7, PC] } }))
Loading from address : BoolectorExpr(#x201fffdc)
Loading from address : 538968028
Loaded value Ok(BoolectorExpr((declare-fun BTOR_2@R5 () (_ BitVec 32))
)) from memory
Loading from address : BoolectorExpr(#x201fffe0)
Loading from address : 538968032
Loaded value Ok(BoolectorExpr(#x201ffff8)) from memory
Loading from address : BoolectorExpr(#x201fffe4)
Loading from address : 538968036
Loaded value Ok(BoolectorExpr(#x20011c01)) from memory
Loading from address : BoolectorExpr(#x201fffe8)
Loading from address : 538968040
Loaded value Ok(BoolectorExpr((concat (concat #x0001 (select memory #x201fffe9)) (select memory #x201fffe8)))) from memory
