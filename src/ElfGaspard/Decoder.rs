
/*  Decoder of instr
    Copyright (C) 2024  Gaspard COURCHINOUX

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/



use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[wasm_bindgen]

pub fn add_instr_to_dom(s :&str) -> Result<(), JsValue> {
  
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html(s);

    body.append_child(&val)?;
    Ok(())
}

 use core::mem::size_of_val;
const MATCH_ADD: u32 = 0x33;
const MASK_ADD: u32 = 0xfe00707f;
const MATCH_ADD16: u32 = 0x40000077;
const MASK_ADD16: u32 = 0xfe00707f;
const MATCH_ADD32: u32 = 0x40002077;
const MASK_ADD32: u32 = 0xfe00707f;
const MATCH_ADD64: u32 = 0xc0001077;
const MASK_ADD64: u32 = 0xfe00707f;
const MATCH_ADD8: u32 = 0x48000077;
const MASK_ADD8: u32 = 0xfe00707f;
const MATCH_ADD_UW: u32 = 0x800003b;
const MASK_ADD_UW: u32 = 0xfe00707f;
const MATCH_ADDD: u32 = 0x7b;
const MASK_ADDD: u32 = 0xfe00707f;
const MATCH_ADDI: u32 = 0x13;
const MASK_ADDI: u32 = 0x707f;
const MATCH_ADDID: u32 = 0x5b;
const MASK_ADDID: u32 = 0x707f;
const MATCH_ADDIW: u32 = 0x1b;
const MASK_ADDIW: u32 = 0x707f;
const MATCH_ADDW: u32 = 0x3b;
const MASK_ADDW: u32 = 0xfe00707f;
const MATCH_AES32DSI: u32 = 0x2a000033;
const MASK_AES32DSI: u32 = 0x3e00707f;
const MATCH_AES32DSMI: u32 = 0x2e000033;
const MASK_AES32DSMI: u32 = 0x3e00707f;
const MATCH_AES32ESI: u32 = 0x22000033;
const MASK_AES32ESI: u32 = 0x3e00707f;
const MATCH_AES32ESMI: u32 = 0x26000033;
const MASK_AES32ESMI: u32 = 0x3e00707f;
const MATCH_AES64DS: u32 = 0x3a000033;
const MASK_AES64DS: u32 = 0xfe00707f;
const MATCH_AES64DSM: u32 = 0x3e000033;
const MASK_AES64DSM: u32 = 0xfe00707f;
const MATCH_AES64ES: u32 = 0x32000033;
const MASK_AES64ES: u32 = 0xfe00707f;
const MATCH_AES64ESM: u32 = 0x36000033;
const MASK_AES64ESM: u32 = 0xfe00707f;
const MATCH_AES64IM: u32 = 0x30001013;
const MASK_AES64IM: u32 = 0xfff0707f;
const MATCH_AES64KS1I: u32 = 0x31001013;
const MASK_AES64KS1I: u32 = 0xff00707f;
const MATCH_AES64KS2: u32 = 0x7e000033;
const MASK_AES64KS2: u32 = 0xfe00707f;
const MATCH_AMOADD_B: u32 = 0x2f;
const MASK_AMOADD_B: u32 = 0xf800707f;
const MATCH_AMOADD_D: u32 = 0x302f;
const MASK_AMOADD_D: u32 = 0xf800707f;
const MATCH_AMOADD_H: u32 = 0x102f;
const MASK_AMOADD_H: u32 = 0xf800707f;
const MATCH_AMOADD_W: u32 = 0x202f;
const MASK_AMOADD_W: u32 = 0xf800707f;
const MATCH_AMOAND_B: u32 = 0x6000002f;
const MASK_AMOAND_B: u32 = 0xf800707f;
const MATCH_AMOAND_D: u32 = 0x6000302f;
const MASK_AMOAND_D: u32 = 0xf800707f;
const MATCH_AMOAND_H: u32 = 0x6000102f;
const MASK_AMOAND_H: u32 = 0xf800707f;
const MATCH_AMOAND_W: u32 = 0x6000202f;
const MASK_AMOAND_W: u32 = 0xf800707f;
const MATCH_AMOCAS_B: u32 = 0x2800002f;
const MASK_AMOCAS_B: u32 = 0xf800707f;
const MATCH_AMOCAS_D: u32 = 0x2800302f;
const MASK_AMOCAS_D: u32 = 0xf800707f;
const MATCH_AMOCAS_H: u32 = 0x2800102f;
const MASK_AMOCAS_H: u32 = 0xf800707f;
const MATCH_AMOCAS_Q: u32 = 0x2800402f;
const MASK_AMOCAS_Q: u32 = 0xf800707f;
const MATCH_AMOCAS_W: u32 = 0x2800202f;
const MASK_AMOCAS_W: u32 = 0xf800707f;
const MATCH_AMOMAX_B: u32 = 0xa000002f;
const MASK_AMOMAX_B: u32 = 0xf800707f;
const MATCH_AMOMAX_D: u32 = 0xa000302f;
const MASK_AMOMAX_D: u32 = 0xf800707f;
const MATCH_AMOMAX_H: u32 = 0xa000102f;
const MASK_AMOMAX_H: u32 = 0xf800707f;
const MATCH_AMOMAX_W: u32 = 0xa000202f;
const MASK_AMOMAX_W: u32 = 0xf800707f;
const MATCH_AMOMAXU_B: u32 = 0xe000002f;
const MASK_AMOMAXU_B: u32 = 0xf800707f;
const MATCH_AMOMAXU_D: u32 = 0xe000302f;
const MASK_AMOMAXU_D: u32 = 0xf800707f;
const MATCH_AMOMAXU_H: u32 = 0xe000102f;
const MASK_AMOMAXU_H: u32 = 0xf800707f;
const MATCH_AMOMAXU_W: u32 = 0xe000202f;
const MASK_AMOMAXU_W: u32 = 0xf800707f;
const MATCH_AMOMIN_B: u32 = 0x8000002f;
const MASK_AMOMIN_B: u32 = 0xf800707f;
const MATCH_AMOMIN_D: u32 = 0x8000302f;
const MASK_AMOMIN_D: u32 = 0xf800707f;
const MATCH_AMOMIN_H: u32 = 0x8000102f;
const MASK_AMOMIN_H: u32 = 0xf800707f;
const MATCH_AMOMIN_W: u32 = 0x8000202f;
const MASK_AMOMIN_W: u32 = 0xf800707f;
const MATCH_AMOMINU_B: u32 = 0xc000002f;
const MASK_AMOMINU_B: u32 = 0xf800707f;
const MATCH_AMOMINU_D: u32 = 0xc000302f;
const MASK_AMOMINU_D: u32 = 0xf800707f;
const MATCH_AMOMINU_H: u32 = 0xc000102f;
const MASK_AMOMINU_H: u32 = 0xf800707f;
const MATCH_AMOMINU_W: u32 = 0xc000202f;
const MASK_AMOMINU_W: u32 = 0xf800707f;
const MATCH_AMOOR_B: u32 = 0x4000002f;
const MASK_AMOOR_B: u32 = 0xf800707f;
const MATCH_AMOOR_D: u32 = 0x4000302f;
const MASK_AMOOR_D: u32 = 0xf800707f;
const MATCH_AMOOR_H: u32 = 0x4000102f;
const MASK_AMOOR_H: u32 = 0xf800707f;
const MATCH_AMOOR_W: u32 = 0x4000202f;
const MASK_AMOOR_W: u32 = 0xf800707f;
const MATCH_AMOSWAP_B: u32 = 0x800002f;
const MASK_AMOSWAP_B: u32 = 0xf800707f;
const MATCH_AMOSWAP_D: u32 = 0x800302f;
const MASK_AMOSWAP_D: u32 = 0xf800707f;
const MATCH_AMOSWAP_H: u32 = 0x800102f;
const MASK_AMOSWAP_H: u32 = 0xf800707f;
const MATCH_AMOSWAP_W: u32 = 0x800202f;
const MASK_AMOSWAP_W: u32 = 0xf800707f;
const MATCH_AMOXOR_B: u32 = 0x2000002f;
const MASK_AMOXOR_B: u32 = 0xf800707f;
const MATCH_AMOXOR_D: u32 = 0x2000302f;
const MASK_AMOXOR_D: u32 = 0xf800707f;
const MATCH_AMOXOR_H: u32 = 0x2000102f;
const MASK_AMOXOR_H: u32 = 0xf800707f;
const MATCH_AMOXOR_W: u32 = 0x2000202f;
const MASK_AMOXOR_W: u32 = 0xf800707f;
const MATCH_AND: u32 = 0x7033;
const MASK_AND: u32 = 0xfe00707f;
const MATCH_ANDI: u32 = 0x7013;
const MASK_ANDI: u32 = 0x707f;
const MATCH_ANDN: u32 = 0x40007033;
const MASK_ANDN: u32 = 0xfe00707f;
const MATCH_AUIPC: u32 = 0x17;
const MASK_AUIPC: u32 = 0x7f;
const MATCH_AVE: u32 = 0xe0000077;
const MASK_AVE: u32 = 0xfe00707f;
const MATCH_BCLR: u32 = 0x48001033;
const MASK_BCLR: u32 = 0xfe00707f;
const MATCH_BCLRI: u32 = 0x48001013;
const MASK_BCLRI: u32 = 0xfc00707f;
const MATCH_BCLRI_RV32: u32 = 0x48001013;
const MASK_BCLRI_RV32: u32 = 0xfe00707f;
const MATCH_BCOMPRESS: u32 = 0x8006033;
const MASK_BCOMPRESS: u32 = 0xfe00707f;
const MATCH_BCOMPRESSW: u32 = 0x800603b;
const MASK_BCOMPRESSW: u32 = 0xfe00707f;
const MATCH_BDECOMPRESS: u32 = 0x48006033;
const MASK_BDECOMPRESS: u32 = 0xfe00707f;
const MATCH_BDECOMPRESSW: u32 = 0x4800603b;
const MASK_BDECOMPRESSW: u32 = 0xfe00707f;
const MATCH_BEQ: u32 = 0x63;
const MASK_BEQ: u32 = 0x707f;
const MATCH_BEXT: u32 = 0x48005033;
const MASK_BEXT: u32 = 0xfe00707f;
const MATCH_BEXTI: u32 = 0x48005013;
const MASK_BEXTI: u32 = 0xfc00707f;
const MATCH_BEXTI_RV32: u32 = 0x48005013;
const MASK_BEXTI_RV32: u32 = 0xfe00707f;
const MATCH_BFP: u32 = 0x48007033;
const MASK_BFP: u32 = 0xfe00707f;
const MATCH_BFPW: u32 = 0x4800703b;
const MASK_BFPW: u32 = 0xfe00707f;
const MATCH_BGE: u32 = 0x5063;
const MASK_BGE: u32 = 0x707f;
const MATCH_BGEU: u32 = 0x7063;
const MASK_BGEU: u32 = 0x707f;
const MATCH_BINV: u32 = 0x68001033;
const MASK_BINV: u32 = 0xfe00707f;
const MATCH_BINVI: u32 = 0x68001013;
const MASK_BINVI: u32 = 0xfc00707f;
const MATCH_BINVI_RV32: u32 = 0x68001013;
const MASK_BINVI_RV32: u32 = 0xfe00707f;
const MATCH_BLT: u32 = 0x4063;
const MASK_BLT: u32 = 0x707f;
const MATCH_BLTU: u32 = 0x6063;
const MASK_BLTU: u32 = 0x707f;
const MATCH_BMATFLIP: u32 = 0x60301013;
const MASK_BMATFLIP: u32 = 0xfff0707f;
const MATCH_BMATOR: u32 = 0x8003033;
const MASK_BMATOR: u32 = 0xfe00707f;
const MATCH_BMATXOR: u32 = 0x48003033;
const MASK_BMATXOR: u32 = 0xfe00707f;
const MATCH_BNE: u32 = 0x1063;
const MASK_BNE: u32 = 0x707f;
const MATCH_BREV8: u32 = 0x68705013;
const MASK_BREV8: u32 = 0xfff0707f;
const MATCH_BSET: u32 = 0x28001033;
const MASK_BSET: u32 = 0xfe00707f;
const MATCH_BSETI: u32 = 0x28001013;
const MASK_BSETI: u32 = 0xfc00707f;
const MATCH_BSETI_RV32: u32 = 0x28001013;
const MASK_BSETI_RV32: u32 = 0xfe00707f;
const MATCH_C_ADD: u32 = 0x9002;
const MASK_C_ADD: u32 = 0xf003;
const MATCH_C_ADDI: u32 = 0x1;
const MASK_C_ADDI: u32 = 0xe003;
const MATCH_C_ADDI16SP: u32 = 0x6101;
const MASK_C_ADDI16SP: u32 = 0xef83;
const MATCH_C_ADDI4SPN: u32 = 0x0;
const MASK_C_ADDI4SPN: u32 = 0xe003;
const MATCH_C_ADDIW: u32 = 0x2001;
const MASK_C_ADDIW: u32 = 0xe003;
const MATCH_C_ADDW: u32 = 0x9c21;
const MASK_C_ADDW: u32 = 0xfc63;
const MATCH_C_AND: u32 = 0x8c61;
const MASK_C_AND: u32 = 0xfc63;
const MATCH_C_ANDI: u32 = 0x8801;
const MASK_C_ANDI: u32 = 0xec03;
const MATCH_C_BEQZ: u32 = 0xc001;
const MASK_C_BEQZ: u32 = 0xe003;
const MATCH_C_BNEZ: u32 = 0xe001;
const MASK_C_BNEZ: u32 = 0xe003;
const MATCH_C_EBREAK: u32 = 0x9002;
const MASK_C_EBREAK: u32 = 0xffff;
const MATCH_C_FLD: u32 = 0x2000;
const MASK_C_FLD: u32 = 0xe003;
const MATCH_C_FLDSP: u32 = 0x2002;
const MASK_C_FLDSP: u32 = 0xe003;
const MATCH_C_FLW: u32 = 0x6000;
const MASK_C_FLW: u32 = 0xe003;
const MATCH_C_FLWSP: u32 = 0x6002;
const MASK_C_FLWSP: u32 = 0xe003;
const MATCH_C_FSD: u32 = 0xa000;
const MASK_C_FSD: u32 = 0xe003;
const MATCH_C_FSDSP: u32 = 0xa002;
const MASK_C_FSDSP: u32 = 0xe003;
const MATCH_C_FSW: u32 = 0xe000;
const MASK_C_FSW: u32 = 0xe003;
const MATCH_C_FSWSP: u32 = 0xe002;
const MASK_C_FSWSP: u32 = 0xe003;
const MATCH_C_J: u32 = 0xa001;
const MASK_C_J: u32 = 0xe003;
const MATCH_C_JAL: u32 = 0x2001;
const MASK_C_JAL: u32 = 0xe003;
const MATCH_C_JALR: u32 = 0x9002;
const MASK_C_JALR: u32 = 0xf07f;
const MATCH_C_JR: u32 = 0x8002;
const MASK_C_JR: u32 = 0xf07f;
const MATCH_C_LBU: u32 = 0x8000;
const MASK_C_LBU: u32 = 0xfc03;
const MATCH_C_LD: u32 = 0x6000;
const MASK_C_LD: u32 = 0xe003;
const MATCH_C_LDSP: u32 = 0x6002;
const MASK_C_LDSP: u32 = 0xe003;
const MATCH_C_LH: u32 = 0x8440;
const MASK_C_LH: u32 = 0xfc43;
const MATCH_C_LHU: u32 = 0x8400;
const MASK_C_LHU: u32 = 0xfc43;
const MATCH_C_LI: u32 = 0x4001;
const MASK_C_LI: u32 = 0xe003;
const MATCH_C_LQ: u32 = 0x2000;
const MASK_C_LQ: u32 = 0xe003;
const MATCH_C_LQSP: u32 = 0x2002;
const MASK_C_LQSP: u32 = 0xe003;
const MATCH_C_LUI: u32 = 0x6001;
const MASK_C_LUI: u32 = 0xe003;
const MATCH_C_LW: u32 = 0x4000;
const MASK_C_LW: u32 = 0xe003;
const MATCH_C_LWSP: u32 = 0x4002;
const MASK_C_LWSP: u32 = 0xe003;
const MATCH_C_MOP_1: u32 = 0x6081;
const MASK_C_MOP_1: u32 = 0xffff;
const MATCH_C_MOP_11: u32 = 0x6581;
const MASK_C_MOP_11: u32 = 0xffff;
const MATCH_C_MOP_13: u32 = 0x6681;
const MASK_C_MOP_13: u32 = 0xffff;
const MATCH_C_MOP_15: u32 = 0x6781;
const MASK_C_MOP_15: u32 = 0xffff;
const MATCH_C_MOP_3: u32 = 0x6181;
const MASK_C_MOP_3: u32 = 0xffff;
const MATCH_C_MOP_5: u32 = 0x6281;
const MASK_C_MOP_5: u32 = 0xffff;
const MATCH_C_MOP_7: u32 = 0x6381;
const MASK_C_MOP_7: u32 = 0xffff;
const MATCH_C_MOP_9: u32 = 0x6481;
const MASK_C_MOP_9: u32 = 0xffff;
const MATCH_C_MOP_N: u32 = 0x6081;
const MASK_C_MOP_N: u32 = 0xf8ff;
const MATCH_C_MUL: u32 = 0x9c41;
const MASK_C_MUL: u32 = 0xfc63;
const MATCH_C_MV: u32 = 0x8002;
const MASK_C_MV: u32 = 0xf003;
const MATCH_C_NOP: u32 = 0x1;
const MASK_C_NOP: u32 = 0xef83;
const MATCH_C_NOT: u32 = 0x9c75;
const MASK_C_NOT: u32 = 0xfc7f;
const MATCH_C_NTL_ALL: u32 = 0x9016;
const MASK_C_NTL_ALL: u32 = 0xffff;
const MATCH_C_NTL_P1: u32 = 0x900a;
const MASK_C_NTL_P1: u32 = 0xffff;
const MATCH_C_NTL_PALL: u32 = 0x900e;
const MASK_C_NTL_PALL: u32 = 0xffff;
const MATCH_C_NTL_S1: u32 = 0x9012;
const MASK_C_NTL_S1: u32 = 0xffff;
const MATCH_C_OR: u32 = 0x8c41;
const MASK_C_OR: u32 = 0xfc63;
const MATCH_C_SB: u32 = 0x8800;
const MASK_C_SB: u32 = 0xfc03;
const MATCH_C_SD: u32 = 0xe000;
const MASK_C_SD: u32 = 0xe003;
const MATCH_C_SDSP: u32 = 0xe002;
const MASK_C_SDSP: u32 = 0xe003;
const MATCH_C_SEXT_B: u32 = 0x9c65;
const MASK_C_SEXT_B: u32 = 0xfc7f;
const MATCH_C_SEXT_H: u32 = 0x9c6d;
const MASK_C_SEXT_H: u32 = 0xfc7f;
const MATCH_C_SH: u32 = 0x8c00;
const MASK_C_SH: u32 = 0xfc43;
const MATCH_C_SLLI: u32 = 0x2;
const MASK_C_SLLI: u32 = 0xe003;
const MATCH_C_SLLI_RV32: u32 = 0x2;
const MASK_C_SLLI_RV32: u32 = 0xf003;
const MATCH_C_SQ: u32 = 0xa000;
const MASK_C_SQ: u32 = 0xe003;
const MATCH_C_SQSP: u32 = 0xa002;
const MASK_C_SQSP: u32 = 0xe003;
const MATCH_C_SRAI: u32 = 0x8401;
const MASK_C_SRAI: u32 = 0xec03;
const MATCH_C_SRAI_RV32: u32 = 0x8401;
const MASK_C_SRAI_RV32: u32 = 0xfc03;
const MATCH_C_SRLI: u32 = 0x8001;
const MASK_C_SRLI: u32 = 0xec03;
const MATCH_C_SRLI_RV32: u32 = 0x8001;
const MASK_C_SRLI_RV32: u32 = 0xfc03;
const MATCH_C_SSPOPCHK_X5: u32 = 0x6281;
const MASK_C_SSPOPCHK_X5: u32 = 0xffff;
const MATCH_C_SSPUSH_X1: u32 = 0x6081;
const MASK_C_SSPUSH_X1: u32 = 0xffff;
const MATCH_C_SUB: u32 = 0x8c01;
const MASK_C_SUB: u32 = 0xfc63;
const MATCH_C_SUBW: u32 = 0x9c01;
const MASK_C_SUBW: u32 = 0xfc63;
const MATCH_C_SW: u32 = 0xc000;
const MASK_C_SW: u32 = 0xe003;
const MATCH_C_SWSP: u32 = 0xc002;
const MASK_C_SWSP: u32 = 0xe003;
const MATCH_C_XOR: u32 = 0x8c21;
const MASK_C_XOR: u32 = 0xfc63;
const MATCH_C_ZEXT_B: u32 = 0x9c61;
const MASK_C_ZEXT_B: u32 = 0xfc7f;
const MATCH_C_ZEXT_H: u32 = 0x9c69;
const MASK_C_ZEXT_H: u32 = 0xfc7f;
const MATCH_C_ZEXT_W: u32 = 0x9c71;
const MASK_C_ZEXT_W: u32 = 0xfc7f;
const MATCH_CBO_CLEAN: u32 = 0x10200f;
const MASK_CBO_CLEAN: u32 = 0xfff07fff;
const MATCH_CBO_FLUSH: u32 = 0x20200f;
const MASK_CBO_FLUSH: u32 = 0xfff07fff;
const MATCH_CBO_INVAL: u32 = 0x200f;
const MASK_CBO_INVAL: u32 = 0xfff07fff;
const MATCH_CBO_ZERO: u32 = 0x40200f;
const MASK_CBO_ZERO: u32 = 0xfff07fff;
const MATCH_CLMUL: u32 = 0xa001033;
const MASK_CLMUL: u32 = 0xfe00707f;
const MATCH_CLMULH: u32 = 0xa003033;
const MASK_CLMULH: u32 = 0xfe00707f;
const MATCH_CLMULR: u32 = 0xa002033;
const MASK_CLMULR: u32 = 0xfe00707f;
const MATCH_CLROV: u32 = 0x90f073;
const MASK_CLROV: u32 = 0xfffff07f;
const MATCH_CLRS16: u32 = 0xae800077;
const MASK_CLRS16: u32 = 0xfff0707f;
const MATCH_CLRS32: u32 = 0xaf800077;
const MASK_CLRS32: u32 = 0xfff0707f;
const MATCH_CLRS8: u32 = 0xae000077;
const MASK_CLRS8: u32 = 0xfff0707f;
const MATCH_CLZ: u32 = 0x60001013;
const MASK_CLZ: u32 = 0xfff0707f;
const MATCH_CLZ16: u32 = 0xae900077;
const MASK_CLZ16: u32 = 0xfff0707f;
const MATCH_CLZ32: u32 = 0xaf900077;
const MASK_CLZ32: u32 = 0xfff0707f;
const MATCH_CLZ8: u32 = 0xae100077;
const MASK_CLZ8: u32 = 0xfff0707f;
const MATCH_CLZW: u32 = 0x6000101b;
const MASK_CLZW: u32 = 0xfff0707f;
const MATCH_CM_JALT: u32 = 0xa002;
const MASK_CM_JALT: u32 = 0xfc03;
const MATCH_CM_MVA01S: u32 = 0xac62;
const MASK_CM_MVA01S: u32 = 0xfc63;
const MATCH_CM_MVSA01: u32 = 0xac22;
const MASK_CM_MVSA01: u32 = 0xfc63;
const MATCH_CM_POP: u32 = 0xba02;
const MASK_CM_POP: u32 = 0xff03;
const MATCH_CM_POPRET: u32 = 0xbe02;
const MASK_CM_POPRET: u32 = 0xff03;
const MATCH_CM_POPRETZ: u32 = 0xbc02;
const MASK_CM_POPRETZ: u32 = 0xff03;
const MATCH_CM_PUSH: u32 = 0xb802;
const MASK_CM_PUSH: u32 = 0xff03;
const MATCH_CMIX: u32 = 0x6001033;
const MASK_CMIX: u32 = 0x600707f;
const MATCH_CMOV: u32 = 0x6005033;
const MASK_CMOV: u32 = 0x600707f;
const MATCH_CMPEQ16: u32 = 0x4c000077;
const MASK_CMPEQ16: u32 = 0xfe00707f;
const MATCH_CMPEQ8: u32 = 0x4e000077;
const MASK_CMPEQ8: u32 = 0xfe00707f;
const MATCH_CPOP: u32 = 0x60201013;
const MASK_CPOP: u32 = 0xfff0707f;
const MATCH_CPOPW: u32 = 0x6020101b;
const MASK_CPOPW: u32 = 0xfff0707f;
const MATCH_CRAS16: u32 = 0x44000077;
const MASK_CRAS16: u32 = 0xfe00707f;
const MATCH_CRAS32: u32 = 0x44002077;
const MASK_CRAS32: u32 = 0xfe00707f;
const MATCH_CRC32_B: u32 = 0x61001013;
const MASK_CRC32_B: u32 = 0xfff0707f;
const MATCH_CRC32_D: u32 = 0x61301013;
const MASK_CRC32_D: u32 = 0xfff0707f;
const MATCH_CRC32_H: u32 = 0x61101013;
const MASK_CRC32_H: u32 = 0xfff0707f;
const MATCH_CRC32_W: u32 = 0x61201013;
const MASK_CRC32_W: u32 = 0xfff0707f;
const MATCH_CRC32C_B: u32 = 0x61801013;
const MASK_CRC32C_B: u32 = 0xfff0707f;
const MATCH_CRC32C_D: u32 = 0x61b01013;
const MASK_CRC32C_D: u32 = 0xfff0707f;
const MATCH_CRC32C_H: u32 = 0x61901013;
const MASK_CRC32C_H: u32 = 0xfff0707f;
const MATCH_CRC32C_W: u32 = 0x61a01013;
const MASK_CRC32C_W: u32 = 0xfff0707f;
const MATCH_CRSA16: u32 = 0x46000077;
const MASK_CRSA16: u32 = 0xfe00707f;
const MATCH_CRSA32: u32 = 0x46002077;
const MASK_CRSA32: u32 = 0xfe00707f;
const MATCH_CSRRC: u32 = 0x3073;
const MASK_CSRRC: u32 = 0x707f;
const MATCH_CSRRCI: u32 = 0x7073;
const MASK_CSRRCI: u32 = 0x707f;
const MATCH_CSRRS: u32 = 0x2073;
const MASK_CSRRS: u32 = 0x707f;
const MATCH_CSRRSI: u32 = 0x6073;
const MASK_CSRRSI: u32 = 0x707f;
const MATCH_CSRRW: u32 = 0x1073;
const MASK_CSRRW: u32 = 0x707f;
const MATCH_CSRRWI: u32 = 0x5073;
const MASK_CSRRWI: u32 = 0x707f;
const MATCH_CTZ: u32 = 0x60101013;
const MASK_CTZ: u32 = 0xfff0707f;
const MATCH_CTZW: u32 = 0x6010101b;
const MASK_CTZW: u32 = 0xfff0707f;
const MATCH_CZERO_EQZ: u32 = 0xe005033;
const MASK_CZERO_EQZ: u32 = 0xfe00707f;
const MATCH_CZERO_NEZ: u32 = 0xe007033;
const MASK_CZERO_NEZ: u32 = 0xfe00707f;
const MATCH_DIV: u32 = 0x2004033;
const MASK_DIV: u32 = 0xfe00707f;
const MATCH_DIVU: u32 = 0x2005033;
const MASK_DIVU: u32 = 0xfe00707f;
const MATCH_DIVUW: u32 = 0x200503b;
const MASK_DIVUW: u32 = 0xfe00707f;
const MATCH_DIVW: u32 = 0x200403b;
const MASK_DIVW: u32 = 0xfe00707f;
const MATCH_DRET: u32 = 0x7b200073;
const MASK_DRET: u32 = 0xffffffff;
const MATCH_EBREAK: u32 = 0x100073;
const MASK_EBREAK: u32 = 0xffffffff;
const MATCH_ECALL: u32 = 0x73;
const MASK_ECALL: u32 = 0xffffffff;
const MATCH_FADD_D: u32 = 0x2000053;
const MASK_FADD_D: u32 = 0xfe00007f;
const MATCH_FADD_H: u32 = 0x4000053;
const MASK_FADD_H: u32 = 0xfe00007f;
const MATCH_FADD_Q: u32 = 0x6000053;
const MASK_FADD_Q: u32 = 0xfe00007f;
const MATCH_FADD_S: u32 = 0x53;
const MASK_FADD_S: u32 = 0xfe00007f;
const MATCH_FCLASS_D: u32 = 0xe2001053;
const MASK_FCLASS_D: u32 = 0xfff0707f;
const MATCH_FCLASS_H: u32 = 0xe4001053;
const MASK_FCLASS_H: u32 = 0xfff0707f;
const MATCH_FCLASS_Q: u32 = 0xe6001053;
const MASK_FCLASS_Q: u32 = 0xfff0707f;
const MATCH_FCLASS_S: u32 = 0xe0001053;
const MASK_FCLASS_S: u32 = 0xfff0707f;
const MATCH_FCVT_BF16_S: u32 = 0x44800053;
const MASK_FCVT_BF16_S: u32 = 0xfff0007f;
const MATCH_FCVT_D_H: u32 = 0x42200053;
const MASK_FCVT_D_H: u32 = 0xfff0007f;
const MATCH_FCVT_D_L: u32 = 0xd2200053;
const MASK_FCVT_D_L: u32 = 0xfff0007f;
const MATCH_FCVT_D_LU: u32 = 0xd2300053;
const MASK_FCVT_D_LU: u32 = 0xfff0007f;
const MATCH_FCVT_D_Q: u32 = 0x42300053;
const MASK_FCVT_D_Q: u32 = 0xfff0007f;
const MATCH_FCVT_D_S: u32 = 0x42000053;
const MASK_FCVT_D_S: u32 = 0xfff0007f;
const MATCH_FCVT_D_W: u32 = 0xd2000053;
const MASK_FCVT_D_W: u32 = 0xfff0007f;
const MATCH_FCVT_D_WU: u32 = 0xd2100053;
const MASK_FCVT_D_WU: u32 = 0xfff0007f;
const MATCH_FCVT_H_D: u32 = 0x44100053;
const MASK_FCVT_H_D: u32 = 0xfff0007f;
const MATCH_FCVT_H_L: u32 = 0xd4200053;
const MASK_FCVT_H_L: u32 = 0xfff0007f;
const MATCH_FCVT_H_LU: u32 = 0xd4300053;
const MASK_FCVT_H_LU: u32 = 0xfff0007f;
const MATCH_FCVT_H_Q: u32 = 0x44300053;
const MASK_FCVT_H_Q: u32 = 0xfff0007f;
const MATCH_FCVT_H_S: u32 = 0x44000053;
const MASK_FCVT_H_S: u32 = 0xfff0007f;
const MATCH_FCVT_H_W: u32 = 0xd4000053;
const MASK_FCVT_H_W: u32 = 0xfff0007f;
const MATCH_FCVT_H_WU: u32 = 0xd4100053;
const MASK_FCVT_H_WU: u32 = 0xfff0007f;
const MATCH_FCVT_L_D: u32 = 0xc2200053;
const MASK_FCVT_L_D: u32 = 0xfff0007f;
const MATCH_FCVT_L_H: u32 = 0xc4200053;
const MASK_FCVT_L_H: u32 = 0xfff0007f;
const MATCH_FCVT_L_Q: u32 = 0xc6200053;
const MASK_FCVT_L_Q: u32 = 0xfff0007f;
const MATCH_FCVT_L_S: u32 = 0xc0200053;
const MASK_FCVT_L_S: u32 = 0xfff0007f;
const MATCH_FCVT_LU_D: u32 = 0xc2300053;
const MASK_FCVT_LU_D: u32 = 0xfff0007f;
const MATCH_FCVT_LU_H: u32 = 0xc4300053;
const MASK_FCVT_LU_H: u32 = 0xfff0007f;
const MATCH_FCVT_LU_Q: u32 = 0xc6300053;
const MASK_FCVT_LU_Q: u32 = 0xfff0007f;
const MATCH_FCVT_LU_S: u32 = 0xc0300053;
const MASK_FCVT_LU_S: u32 = 0xfff0007f;
const MATCH_FCVT_Q_D: u32 = 0x46100053;
const MASK_FCVT_Q_D: u32 = 0xfff0007f;
const MATCH_FCVT_Q_H: u32 = 0x46200053;
const MASK_FCVT_Q_H: u32 = 0xfff0007f;
const MATCH_FCVT_Q_L: u32 = 0xd6200053;
const MASK_FCVT_Q_L: u32 = 0xfff0007f;
const MATCH_FCVT_Q_LU: u32 = 0xd6300053;
const MASK_FCVT_Q_LU: u32 = 0xfff0007f;
const MATCH_FCVT_Q_S: u32 = 0x46000053;
const MASK_FCVT_Q_S: u32 = 0xfff0007f;
const MATCH_FCVT_Q_W: u32 = 0xd6000053;
const MASK_FCVT_Q_W: u32 = 0xfff0007f;
const MATCH_FCVT_Q_WU: u32 = 0xd6100053;
const MASK_FCVT_Q_WU: u32 = 0xfff0007f;
const MATCH_FCVT_S_BF16: u32 = 0x40600053;
const MASK_FCVT_S_BF16: u32 = 0xfff0007f;
const MATCH_FCVT_S_D: u32 = 0x40100053;
const MASK_FCVT_S_D: u32 = 0xfff0007f;
const MATCH_FCVT_S_H: u32 = 0x40200053;
const MASK_FCVT_S_H: u32 = 0xfff0007f;
const MATCH_FCVT_S_L: u32 = 0xd0200053;
const MASK_FCVT_S_L: u32 = 0xfff0007f;
const MATCH_FCVT_S_LU: u32 = 0xd0300053;
const MASK_FCVT_S_LU: u32 = 0xfff0007f;
const MATCH_FCVT_S_Q: u32 = 0x40300053;
const MASK_FCVT_S_Q: u32 = 0xfff0007f;
const MATCH_FCVT_S_W: u32 = 0xd0000053;
const MASK_FCVT_S_W: u32 = 0xfff0007f;
const MATCH_FCVT_S_WU: u32 = 0xd0100053;
const MASK_FCVT_S_WU: u32 = 0xfff0007f;
const MATCH_FCVT_W_D: u32 = 0xc2000053;
const MASK_FCVT_W_D: u32 = 0xfff0007f;
const MATCH_FCVT_W_H: u32 = 0xc4000053;
const MASK_FCVT_W_H: u32 = 0xfff0007f;
const MATCH_FCVT_W_Q: u32 = 0xc6000053;
const MASK_FCVT_W_Q: u32 = 0xfff0007f;
const MATCH_FCVT_W_S: u32 = 0xc0000053;
const MASK_FCVT_W_S: u32 = 0xfff0007f;
const MATCH_FCVT_WU_D: u32 = 0xc2100053;
const MASK_FCVT_WU_D: u32 = 0xfff0007f;
const MATCH_FCVT_WU_H: u32 = 0xc4100053;
const MASK_FCVT_WU_H: u32 = 0xfff0007f;
const MATCH_FCVT_WU_Q: u32 = 0xc6100053;
const MASK_FCVT_WU_Q: u32 = 0xfff0007f;
const MATCH_FCVT_WU_S: u32 = 0xc0100053;
const MASK_FCVT_WU_S: u32 = 0xfff0007f;
const MATCH_FCVTMOD_W_D: u32 = 0xc2801053;
const MASK_FCVTMOD_W_D: u32 = 0xfff0707f;
const MATCH_FDIV_D: u32 = 0x1a000053;
const MASK_FDIV_D: u32 = 0xfe00007f;
const MATCH_FDIV_H: u32 = 0x1c000053;
const MASK_FDIV_H: u32 = 0xfe00007f;
const MATCH_FDIV_Q: u32 = 0x1e000053;
const MASK_FDIV_Q: u32 = 0xfe00007f;
const MATCH_FDIV_S: u32 = 0x18000053;
const MASK_FDIV_S: u32 = 0xfe00007f;
const MATCH_FENCE: u32 = 0xf;
const MASK_FENCE: u32 = 0x707f;
const MATCH_FENCE_I: u32 = 0x100f;
const MASK_FENCE_I: u32 = 0x707f;
const MATCH_FENCE_TSO: u32 = 0x8330000f;
const MASK_FENCE_TSO: u32 = 0xfff0707f;
const MATCH_FEQ_D: u32 = 0xa2002053;
const MASK_FEQ_D: u32 = 0xfe00707f;
const MATCH_FEQ_H: u32 = 0xa4002053;
const MASK_FEQ_H: u32 = 0xfe00707f;
const MATCH_FEQ_Q: u32 = 0xa6002053;
const MASK_FEQ_Q: u32 = 0xfe00707f;
const MATCH_FEQ_S: u32 = 0xa0002053;
const MASK_FEQ_S: u32 = 0xfe00707f;
const MATCH_FLD: u32 = 0x3007;
const MASK_FLD: u32 = 0x707f;
const MATCH_FLE_D: u32 = 0xa2000053;
const MASK_FLE_D: u32 = 0xfe00707f;
const MATCH_FLE_H: u32 = 0xa4000053;
const MASK_FLE_H: u32 = 0xfe00707f;
const MATCH_FLE_Q: u32 = 0xa6000053;
const MASK_FLE_Q: u32 = 0xfe00707f;
const MATCH_FLE_S: u32 = 0xa0000053;
const MASK_FLE_S: u32 = 0xfe00707f;
const MATCH_FLEQ_D: u32 = 0xa2004053;
const MASK_FLEQ_D: u32 = 0xfe00707f;
const MATCH_FLEQ_H: u32 = 0xa4004053;
const MASK_FLEQ_H: u32 = 0xfe00707f;
const MATCH_FLEQ_Q: u32 = 0xa6004053;
const MASK_FLEQ_Q: u32 = 0xfe00707f;
const MATCH_FLEQ_S: u32 = 0xa0004053;
const MASK_FLEQ_S: u32 = 0xfe00707f;
const MATCH_FLH: u32 = 0x1007;
const MASK_FLH: u32 = 0x707f;
const MATCH_FLI_D: u32 = 0xf2100053;
const MASK_FLI_D: u32 = 0xfff0707f;
const MATCH_FLI_H: u32 = 0xf4100053;
const MASK_FLI_H: u32 = 0xfff0707f;
const MATCH_FLI_Q: u32 = 0xf6100053;
const MASK_FLI_Q: u32 = 0xfff0707f;
const MATCH_FLI_S: u32 = 0xf0100053;
const MASK_FLI_S: u32 = 0xfff0707f;
const MATCH_FLQ: u32 = 0x4007;
const MASK_FLQ: u32 = 0x707f;
const MATCH_FLT_D: u32 = 0xa2001053;
const MASK_FLT_D: u32 = 0xfe00707f;
const MATCH_FLT_H: u32 = 0xa4001053;
const MASK_FLT_H: u32 = 0xfe00707f;
const MATCH_FLT_Q: u32 = 0xa6001053;
const MASK_FLT_Q: u32 = 0xfe00707f;
const MATCH_FLT_S: u32 = 0xa0001053;
const MASK_FLT_S: u32 = 0xfe00707f;
const MATCH_FLTQ_D: u32 = 0xa2005053;
const MASK_FLTQ_D: u32 = 0xfe00707f;
const MATCH_FLTQ_H: u32 = 0xa4005053;
const MASK_FLTQ_H: u32 = 0xfe00707f;
const MATCH_FLTQ_Q: u32 = 0xa6005053;
const MASK_FLTQ_Q: u32 = 0xfe00707f;
const MATCH_FLTQ_S: u32 = 0xa0005053;
const MASK_FLTQ_S: u32 = 0xfe00707f;
const MATCH_FLW: u32 = 0x2007;
const MASK_FLW: u32 = 0x707f;
const MATCH_FMADD_D: u32 = 0x2000043;
const MASK_FMADD_D: u32 = 0x600007f;
const MATCH_FMADD_H: u32 = 0x4000043;
const MASK_FMADD_H: u32 = 0x600007f;
const MATCH_FMADD_Q: u32 = 0x6000043;
const MASK_FMADD_Q: u32 = 0x600007f;
const MATCH_FMADD_S: u32 = 0x43;
const MASK_FMADD_S: u32 = 0x600007f;
const MATCH_FMAX_D: u32 = 0x2a001053;
const MASK_FMAX_D: u32 = 0xfe00707f;
const MATCH_FMAX_H: u32 = 0x2c001053;
const MASK_FMAX_H: u32 = 0xfe00707f;
const MATCH_FMAX_Q: u32 = 0x2e001053;
const MASK_FMAX_Q: u32 = 0xfe00707f;
const MATCH_FMAX_S: u32 = 0x28001053;
const MASK_FMAX_S: u32 = 0xfe00707f;
const MATCH_FMAXM_D: u32 = 0x2a003053;
const MASK_FMAXM_D: u32 = 0xfe00707f;
const MATCH_FMAXM_H: u32 = 0x2c003053;
const MASK_FMAXM_H: u32 = 0xfe00707f;
const MATCH_FMAXM_Q: u32 = 0x2e003053;
const MASK_FMAXM_Q: u32 = 0xfe00707f;
const MATCH_FMAXM_S: u32 = 0x28003053;
const MASK_FMAXM_S: u32 = 0xfe00707f;
const MATCH_FMIN_D: u32 = 0x2a000053;
const MASK_FMIN_D: u32 = 0xfe00707f;
const MATCH_FMIN_H: u32 = 0x2c000053;
const MASK_FMIN_H: u32 = 0xfe00707f;
const MATCH_FMIN_Q: u32 = 0x2e000053;
const MASK_FMIN_Q: u32 = 0xfe00707f;
const MATCH_FMIN_S: u32 = 0x28000053;
const MASK_FMIN_S: u32 = 0xfe00707f;
const MATCH_FMINM_D: u32 = 0x2a002053;
const MASK_FMINM_D: u32 = 0xfe00707f;
const MATCH_FMINM_H: u32 = 0x2c002053;
const MASK_FMINM_H: u32 = 0xfe00707f;
const MATCH_FMINM_Q: u32 = 0x2e002053;
const MASK_FMINM_Q: u32 = 0xfe00707f;
const MATCH_FMINM_S: u32 = 0x28002053;
const MASK_FMINM_S: u32 = 0xfe00707f;
const MATCH_FMSUB_D: u32 = 0x2000047;
const MASK_FMSUB_D: u32 = 0x600007f;
const MATCH_FMSUB_H: u32 = 0x4000047;
const MASK_FMSUB_H: u32 = 0x600007f;
const MATCH_FMSUB_Q: u32 = 0x6000047;
const MASK_FMSUB_Q: u32 = 0x600007f;
const MATCH_FMSUB_S: u32 = 0x47;
const MASK_FMSUB_S: u32 = 0x600007f;
const MATCH_FMUL_D: u32 = 0x12000053;
const MASK_FMUL_D: u32 = 0xfe00007f;
const MATCH_FMUL_H: u32 = 0x14000053;
const MASK_FMUL_H: u32 = 0xfe00007f;
const MATCH_FMUL_Q: u32 = 0x16000053;
const MASK_FMUL_Q: u32 = 0xfe00007f;
const MATCH_FMUL_S: u32 = 0x10000053;
const MASK_FMUL_S: u32 = 0xfe00007f;
const MATCH_FMV_D_X: u32 = 0xf2000053;
const MASK_FMV_D_X: u32 = 0xfff0707f;
const MATCH_FMV_H_X: u32 = 0xf4000053;
const MASK_FMV_H_X: u32 = 0xfff0707f;
const MATCH_FMV_S_X: u32 = 0xf0000053;
const MASK_FMV_S_X: u32 = 0xfff0707f;
const MATCH_FMV_W_X: u32 = 0xf0000053;
const MASK_FMV_W_X: u32 = 0xfff0707f;
const MATCH_FMV_X_D: u32 = 0xe2000053;
const MASK_FMV_X_D: u32 = 0xfff0707f;
const MATCH_FMV_X_H: u32 = 0xe4000053;
const MASK_FMV_X_H: u32 = 0xfff0707f;
const MATCH_FMV_X_S: u32 = 0xe0000053;
const MASK_FMV_X_S: u32 = 0xfff0707f;
const MATCH_FMV_X_W: u32 = 0xe0000053;
const MASK_FMV_X_W: u32 = 0xfff0707f;
const MATCH_FMVH_X_D: u32 = 0xe2100053;
const MASK_FMVH_X_D: u32 = 0xfff0707f;
const MATCH_FMVH_X_Q: u32 = 0xe6100053;
const MASK_FMVH_X_Q: u32 = 0xfff0707f;
const MATCH_FMVP_D_X: u32 = 0xb2000053;
const MASK_FMVP_D_X: u32 = 0xfe00707f;
const MATCH_FMVP_Q_X: u32 = 0xb6000053;
const MASK_FMVP_Q_X: u32 = 0xfe00707f;
const MATCH_FNMADD_D: u32 = 0x200004f;
const MASK_FNMADD_D: u32 = 0x600007f;
const MATCH_FNMADD_H: u32 = 0x400004f;
const MASK_FNMADD_H: u32 = 0x600007f;
const MATCH_FNMADD_Q: u32 = 0x600004f;
const MASK_FNMADD_Q: u32 = 0x600007f;
const MATCH_FNMADD_S: u32 = 0x4f;
const MASK_FNMADD_S: u32 = 0x600007f;
const MATCH_FNMSUB_D: u32 = 0x200004b;
const MASK_FNMSUB_D: u32 = 0x600007f;
const MATCH_FNMSUB_H: u32 = 0x400004b;
const MASK_FNMSUB_H: u32 = 0x600007f;
const MATCH_FNMSUB_Q: u32 = 0x600004b;
const MASK_FNMSUB_Q: u32 = 0x600007f;
const MATCH_FNMSUB_S: u32 = 0x4b;
const MASK_FNMSUB_S: u32 = 0x600007f;
const MATCH_FRCSR: u32 = 0x302073;
const MASK_FRCSR: u32 = 0xfffff07f;
const MATCH_FRFLAGS: u32 = 0x102073;
const MASK_FRFLAGS: u32 = 0xfffff07f;
const MATCH_FROUND_D: u32 = 0x42400053;
const MASK_FROUND_D: u32 = 0xfff0007f;
const MATCH_FROUND_H: u32 = 0x44400053;
const MASK_FROUND_H: u32 = 0xfff0007f;
const MATCH_FROUND_Q: u32 = 0x46400053;
const MASK_FROUND_Q: u32 = 0xfff0007f;
const MATCH_FROUND_S: u32 = 0x40400053;
const MASK_FROUND_S: u32 = 0xfff0007f;
const MATCH_FROUNDNX_D: u32 = 0x42500053;
const MASK_FROUNDNX_D: u32 = 0xfff0007f;
const MATCH_FROUNDNX_H: u32 = 0x44500053;
const MASK_FROUNDNX_H: u32 = 0xfff0007f;
const MATCH_FROUNDNX_Q: u32 = 0x46500053;
const MASK_FROUNDNX_Q: u32 = 0xfff0007f;
const MATCH_FROUNDNX_S: u32 = 0x40500053;
const MASK_FROUNDNX_S: u32 = 0xfff0007f;
const MATCH_FRRM: u32 = 0x202073;
const MASK_FRRM: u32 = 0xfffff07f;
const MATCH_FSCSR: u32 = 0x301073;
const MASK_FSCSR: u32 = 0xfff0707f;
const MATCH_FSD: u32 = 0x3027;
const MASK_FSD: u32 = 0x707f;
const MATCH_FSFLAGS: u32 = 0x101073;
const MASK_FSFLAGS: u32 = 0xfff0707f;
const MATCH_FSFLAGSI: u32 = 0x105073;
const MASK_FSFLAGSI: u32 = 0xfff0707f;
const MATCH_FSGNJ_D: u32 = 0x22000053;
const MASK_FSGNJ_D: u32 = 0xfe00707f;
const MATCH_FSGNJ_H: u32 = 0x24000053;
const MASK_FSGNJ_H: u32 = 0xfe00707f;
const MATCH_FSGNJ_Q: u32 = 0x26000053;
const MASK_FSGNJ_Q: u32 = 0xfe00707f;
const MATCH_FSGNJ_S: u32 = 0x20000053;
const MASK_FSGNJ_S: u32 = 0xfe00707f;
const MATCH_FSGNJN_D: u32 = 0x22001053;
const MASK_FSGNJN_D: u32 = 0xfe00707f;
const MATCH_FSGNJN_H: u32 = 0x24001053;
const MASK_FSGNJN_H: u32 = 0xfe00707f;
const MATCH_FSGNJN_Q: u32 = 0x26001053;
const MASK_FSGNJN_Q: u32 = 0xfe00707f;
const MATCH_FSGNJN_S: u32 = 0x20001053;
const MASK_FSGNJN_S: u32 = 0xfe00707f;
const MATCH_FSGNJX_D: u32 = 0x22002053;
const MASK_FSGNJX_D: u32 = 0xfe00707f;
const MATCH_FSGNJX_H: u32 = 0x24002053;
const MASK_FSGNJX_H: u32 = 0xfe00707f;
const MATCH_FSGNJX_Q: u32 = 0x26002053;
const MASK_FSGNJX_Q: u32 = 0xfe00707f;
const MATCH_FSGNJX_S: u32 = 0x20002053;
const MASK_FSGNJX_S: u32 = 0xfe00707f;
const MATCH_FSH: u32 = 0x1027;
const MASK_FSH: u32 = 0x707f;
const MATCH_FSL: u32 = 0x4001033;
const MASK_FSL: u32 = 0x600707f;
const MATCH_FSLW: u32 = 0x400103b;
const MASK_FSLW: u32 = 0x600707f;
const MATCH_FSQ: u32 = 0x4027;
const MASK_FSQ: u32 = 0x707f;
const MATCH_FSQRT_D: u32 = 0x5a000053;
const MASK_FSQRT_D: u32 = 0xfff0007f;
const MATCH_FSQRT_H: u32 = 0x5c000053;
const MASK_FSQRT_H: u32 = 0xfff0007f;
const MATCH_FSQRT_Q: u32 = 0x5e000053;
const MASK_FSQRT_Q: u32 = 0xfff0007f;
const MATCH_FSQRT_S: u32 = 0x58000053;
const MASK_FSQRT_S: u32 = 0xfff0007f;
const MATCH_FSR: u32 = 0x4005033;
const MASK_FSR: u32 = 0x600707f;
const MATCH_FSRI: u32 = 0x4005013;
const MASK_FSRI: u32 = 0x400707f;
const MATCH_FSRIW: u32 = 0x400501b;
const MASK_FSRIW: u32 = 0x600707f;
const MATCH_FSRM: u32 = 0x201073;
const MASK_FSRM: u32 = 0xfff0707f;
const MATCH_FSRMI: u32 = 0x205073;
const MASK_FSRMI: u32 = 0xfff0707f;
const MATCH_FSRW: u32 = 0x400503b;
const MASK_FSRW: u32 = 0x600707f;
const MATCH_FSUB_D: u32 = 0xa000053;
const MASK_FSUB_D: u32 = 0xfe00007f;
const MATCH_FSUB_H: u32 = 0xc000053;
const MASK_FSUB_H: u32 = 0xfe00007f;
const MATCH_FSUB_Q: u32 = 0xe000053;
const MASK_FSUB_Q: u32 = 0xfe00007f;
const MATCH_FSUB_S: u32 = 0x8000053;
const MASK_FSUB_S: u32 = 0xfe00007f;
const MATCH_FSW: u32 = 0x2027;
const MASK_FSW: u32 = 0x707f;
const MATCH_GORC: u32 = 0x28005033;
const MASK_GORC: u32 = 0xfe00707f;
const MATCH_GORCI: u32 = 0x28005013;
const MASK_GORCI: u32 = 0xfc00707f;
const MATCH_GORCIW: u32 = 0x2800501b;
const MASK_GORCIW: u32 = 0xfe00707f;
const MATCH_GORCW: u32 = 0x2800503b;
const MASK_GORCW: u32 = 0xfe00707f;
const MATCH_GREV: u32 = 0x68005033;
const MASK_GREV: u32 = 0xfe00707f;
const MATCH_GREVI: u32 = 0x68005013;
const MASK_GREVI: u32 = 0xfc00707f;
const MATCH_GREVIW: u32 = 0x6800501b;
const MASK_GREVIW: u32 = 0xfe00707f;
const MATCH_GREVW: u32 = 0x6800503b;
const MASK_GREVW: u32 = 0xfe00707f;
const MATCH_HFENCE_GVMA: u32 = 0x62000073;
const MASK_HFENCE_GVMA: u32 = 0xfe007fff;
const MATCH_HFENCE_VVMA: u32 = 0x22000073;
const MASK_HFENCE_VVMA: u32 = 0xfe007fff;
const MATCH_HINVAL_GVMA: u32 = 0x66000073;
const MASK_HINVAL_GVMA: u32 = 0xfe007fff;
const MATCH_HINVAL_VVMA: u32 = 0x26000073;
const MASK_HINVAL_VVMA: u32 = 0xfe007fff;
const MATCH_HLV_B: u32 = 0x60004073;
const MASK_HLV_B: u32 = 0xfff0707f;
const MATCH_HLV_BU: u32 = 0x60104073;
const MASK_HLV_BU: u32 = 0xfff0707f;
const MATCH_HLV_D: u32 = 0x6c004073;
const MASK_HLV_D: u32 = 0xfff0707f;
const MATCH_HLV_H: u32 = 0x64004073;
const MASK_HLV_H: u32 = 0xfff0707f;
const MATCH_HLV_HU: u32 = 0x64104073;
const MASK_HLV_HU: u32 = 0xfff0707f;
const MATCH_HLV_W: u32 = 0x68004073;
const MASK_HLV_W: u32 = 0xfff0707f;
const MATCH_HLV_WU: u32 = 0x68104073;
const MASK_HLV_WU: u32 = 0xfff0707f;
const MATCH_HLVX_HU: u32 = 0x64304073;
const MASK_HLVX_HU: u32 = 0xfff0707f;
const MATCH_HLVX_WU: u32 = 0x68304073;
const MASK_HLVX_WU: u32 = 0xfff0707f;
const MATCH_HSV_B: u32 = 0x62004073;
const MASK_HSV_B: u32 = 0xfe007fff;
const MATCH_HSV_D: u32 = 0x6e004073;
const MASK_HSV_D: u32 = 0xfe007fff;
const MATCH_HSV_H: u32 = 0x66004073;
const MASK_HSV_H: u32 = 0xfe007fff;
const MATCH_HSV_W: u32 = 0x6a004073;
const MASK_HSV_W: u32 = 0xfe007fff;
const MATCH_INSB: u32 = 0xac000077;
const MASK_INSB: u32 = 0xff80707f;
const MATCH_JAL: u32 = 0x6f;
const MASK_JAL: u32 = 0x7f;
const MATCH_JALR: u32 = 0x67;
const MASK_JALR: u32 = 0x707f;
const MATCH_KABS16: u32 = 0xad100077;
const MASK_KABS16: u32 = 0xfff0707f;
const MATCH_KABS32: u32 = 0xad200077;
const MASK_KABS32: u32 = 0xfff0707f;
const MATCH_KABS8: u32 = 0xad000077;
const MASK_KABS8: u32 = 0xfff0707f;
const MATCH_KABSW: u32 = 0xad400077;
const MASK_KABSW: u32 = 0xfff0707f;
const MATCH_KADD16: u32 = 0x10000077;
const MASK_KADD16: u32 = 0xfe00707f;
const MATCH_KADD32: u32 = 0x10002077;
const MASK_KADD32: u32 = 0xfe00707f;
const MATCH_KADD64: u32 = 0x90001077;
const MASK_KADD64: u32 = 0xfe00707f;
const MATCH_KADD8: u32 = 0x18000077;
const MASK_KADD8: u32 = 0xfe00707f;
const MATCH_KADDH: u32 = 0x4001077;
const MASK_KADDH: u32 = 0xfe00707f;
const MATCH_KADDW: u32 = 0x1077;
const MASK_KADDW: u32 = 0xfe00707f;
const MATCH_KCRAS16: u32 = 0x14000077;
const MASK_KCRAS16: u32 = 0xfe00707f;
const MATCH_KCRAS32: u32 = 0x14002077;
const MASK_KCRAS32: u32 = 0xfe00707f;
const MATCH_KCRSA16: u32 = 0x16000077;
const MASK_KCRSA16: u32 = 0xfe00707f;
const MATCH_KCRSA32: u32 = 0x16002077;
const MASK_KCRSA32: u32 = 0xfe00707f;
const MATCH_KDMABB: u32 = 0xd2001077;
const MASK_KDMABB: u32 = 0xfe00707f;
const MATCH_KDMABB16: u32 = 0xd8001077;
const MASK_KDMABB16: u32 = 0xfe00707f;
const MATCH_KDMABT: u32 = 0xe2001077;
const MASK_KDMABT: u32 = 0xfe00707f;
const MATCH_KDMABT16: u32 = 0xe8001077;
const MASK_KDMABT16: u32 = 0xfe00707f;
const MATCH_KDMATT: u32 = 0xf2001077;
const MASK_KDMATT: u32 = 0xfe00707f;
const MATCH_KDMATT16: u32 = 0xf8001077;
const MASK_KDMATT16: u32 = 0xfe00707f;
const MATCH_KDMBB: u32 = 0xa001077;
const MASK_KDMBB: u32 = 0xfe00707f;
const MATCH_KDMBB16: u32 = 0xda001077;
const MASK_KDMBB16: u32 = 0xfe00707f;
const MATCH_KDMBT: u32 = 0x1a001077;
const MASK_KDMBT: u32 = 0xfe00707f;
const MATCH_KDMBT16: u32 = 0xea001077;
const MASK_KDMBT16: u32 = 0xfe00707f;
const MATCH_KDMTT: u32 = 0x2a001077;
const MASK_KDMTT: u32 = 0xfe00707f;
const MATCH_KDMTT16: u32 = 0xfa001077;
const MASK_KDMTT16: u32 = 0xfe00707f;
const MATCH_KHM16: u32 = 0x86000077;
const MASK_KHM16: u32 = 0xfe00707f;
const MATCH_KHM8: u32 = 0x8e000077;
const MASK_KHM8: u32 = 0xfe00707f;
const MATCH_KHMBB: u32 = 0xc001077;
const MASK_KHMBB: u32 = 0xfe00707f;
const MATCH_KHMBB16: u32 = 0xdc001077;
const MASK_KHMBB16: u32 = 0xfe00707f;
const MATCH_KHMBT: u32 = 0x1c001077;
const MASK_KHMBT: u32 = 0xfe00707f;
const MATCH_KHMBT16: u32 = 0xec001077;
const MASK_KHMBT16: u32 = 0xfe00707f;
const MATCH_KHMTT: u32 = 0x2c001077;
const MASK_KHMTT: u32 = 0xfe00707f;
const MATCH_KHMTT16: u32 = 0xfc001077;
const MASK_KHMTT16: u32 = 0xfe00707f;
const MATCH_KHMX16: u32 = 0x96000077;
const MASK_KHMX16: u32 = 0xfe00707f;
const MATCH_KHMX8: u32 = 0x9e000077;
const MASK_KHMX8: u32 = 0xfe00707f;
const MATCH_KMABB: u32 = 0x5a001077;
const MASK_KMABB: u32 = 0xfe00707f;
const MATCH_KMABB32: u32 = 0x5a002077;
const MASK_KMABB32: u32 = 0xfe00707f;
const MATCH_KMABT: u32 = 0x6a001077;
const MASK_KMABT: u32 = 0xfe00707f;
const MATCH_KMABT32: u32 = 0x6a002077;
const MASK_KMABT32: u32 = 0xfe00707f;
const MATCH_KMADA: u32 = 0x48001077;
const MASK_KMADA: u32 = 0xfe00707f;
const MATCH_KMADRS: u32 = 0x6c001077;
const MASK_KMADRS: u32 = 0xfe00707f;
const MATCH_KMADRS32: u32 = 0x6c002077;
const MASK_KMADRS32: u32 = 0xfe00707f;
const MATCH_KMADS: u32 = 0x5c001077;
const MASK_KMADS: u32 = 0xfe00707f;
const MATCH_KMADS32: u32 = 0x5c002077;
const MASK_KMADS32: u32 = 0xfe00707f;
const MATCH_KMAR64: u32 = 0x94001077;
const MASK_KMAR64: u32 = 0xfe00707f;
const MATCH_KMATT: u32 = 0x7a001077;
const MASK_KMATT: u32 = 0xfe00707f;
const MATCH_KMATT32: u32 = 0x7a002077;
const MASK_KMATT32: u32 = 0xfe00707f;
const MATCH_KMAXDA: u32 = 0x4a001077;
const MASK_KMAXDA: u32 = 0xfe00707f;
const MATCH_KMAXDA32: u32 = 0x4a002077;
const MASK_KMAXDA32: u32 = 0xfe00707f;
const MATCH_KMAXDS: u32 = 0x7c001077;
const MASK_KMAXDS: u32 = 0xfe00707f;
const MATCH_KMAXDS32: u32 = 0x7c002077;
const MASK_KMAXDS32: u32 = 0xfe00707f;
const MATCH_KMDA: u32 = 0x38001077;
const MASK_KMDA: u32 = 0xfe00707f;
const MATCH_KMDA32: u32 = 0x38002077;
const MASK_KMDA32: u32 = 0xfe00707f;
const MATCH_KMMAC: u32 = 0x60001077;
const MASK_KMMAC: u32 = 0xfe00707f;
const MATCH_KMMAC_U: u32 = 0x70001077;
const MASK_KMMAC_U: u32 = 0xfe00707f;
const MATCH_KMMAWB: u32 = 0x46001077;
const MASK_KMMAWB: u32 = 0xfe00707f;
const MATCH_KMMAWB2: u32 = 0xce001077;
const MASK_KMMAWB2: u32 = 0xfe00707f;
const MATCH_KMMAWB2_U: u32 = 0xde001077;
const MASK_KMMAWB2_U: u32 = 0xfe00707f;
const MATCH_KMMAWB_U: u32 = 0x56001077;
const MASK_KMMAWB_U: u32 = 0xfe00707f;
const MATCH_KMMAWT: u32 = 0x66001077;
const MASK_KMMAWT: u32 = 0xfe00707f;
const MATCH_KMMAWT2: u32 = 0xee001077;
const MASK_KMMAWT2: u32 = 0xfe00707f;
const MATCH_KMMAWT2_U: u32 = 0xfe001077;
const MASK_KMMAWT2_U: u32 = 0xfe00707f;
const MATCH_KMMAWT_U: u32 = 0x76001077;
const MASK_KMMAWT_U: u32 = 0xfe00707f;
const MATCH_KMMSB: u32 = 0x42001077;
const MASK_KMMSB: u32 = 0xfe00707f;
const MATCH_KMMSB_U: u32 = 0x52001077;
const MASK_KMMSB_U: u32 = 0xfe00707f;
const MATCH_KMMWB2: u32 = 0x8e001077;
const MASK_KMMWB2: u32 = 0xfe00707f;
const MATCH_KMMWB2_U: u32 = 0x9e001077;
const MASK_KMMWB2_U: u32 = 0xfe00707f;
const MATCH_KMMWT2: u32 = 0xae001077;
const MASK_KMMWT2: u32 = 0xfe00707f;
const MATCH_KMMWT2_U: u32 = 0xbe001077;
const MASK_KMMWT2_U: u32 = 0xfe00707f;
const MATCH_KMSDA: u32 = 0x4c001077;
const MASK_KMSDA: u32 = 0xfe00707f;
const MATCH_KMSDA32: u32 = 0x4c002077;
const MASK_KMSDA32: u32 = 0xfe00707f;
const MATCH_KMSR64: u32 = 0x96001077;
const MASK_KMSR64: u32 = 0xfe00707f;
const MATCH_KMSXDA: u32 = 0x4e001077;
const MASK_KMSXDA: u32 = 0xfe00707f;
const MATCH_KMSXDA32: u32 = 0x4e002077;
const MASK_KMSXDA32: u32 = 0xfe00707f;
const MATCH_KMXDA: u32 = 0x3a001077;
const MASK_KMXDA: u32 = 0xfe00707f;
const MATCH_KMXDA32: u32 = 0x3a002077;
const MASK_KMXDA32: u32 = 0xfe00707f;
const MATCH_KSLL16: u32 = 0x64000077;
const MASK_KSLL16: u32 = 0xfe00707f;
const MATCH_KSLL32: u32 = 0x64002077;
const MASK_KSLL32: u32 = 0xfe00707f;
const MATCH_KSLL8: u32 = 0x6c000077;
const MASK_KSLL8: u32 = 0xfe00707f;
const MATCH_KSLLI16: u32 = 0x75000077;
const MASK_KSLLI16: u32 = 0xff00707f;
const MATCH_KSLLI32: u32 = 0x84002077;
const MASK_KSLLI32: u32 = 0xfe00707f;
const MATCH_KSLLI8: u32 = 0x7c800077;
const MASK_KSLLI8: u32 = 0xff80707f;
const MATCH_KSLLIW: u32 = 0x36001077;
const MASK_KSLLIW: u32 = 0xfe00707f;
const MATCH_KSLLW: u32 = 0x26001077;
const MASK_KSLLW: u32 = 0xfe00707f;
const MATCH_KSLRA16: u32 = 0x56000077;
const MASK_KSLRA16: u32 = 0xfe00707f;
const MATCH_KSLRA16_U: u32 = 0x66000077;
const MASK_KSLRA16_U: u32 = 0xfe00707f;
const MATCH_KSLRA32: u32 = 0x56002077;
const MASK_KSLRA32: u32 = 0xfe00707f;
const MATCH_KSLRA32_U: u32 = 0x66002077;
const MASK_KSLRA32_U: u32 = 0xfe00707f;
const MATCH_KSLRA8: u32 = 0x5e000077;
const MASK_KSLRA8: u32 = 0xfe00707f;
const MATCH_KSLRA8_U: u32 = 0x6e000077;
const MASK_KSLRA8_U: u32 = 0xfe00707f;
const MATCH_KSLRAW: u32 = 0x6e001077;
const MASK_KSLRAW: u32 = 0xfe00707f;
const MATCH_KSLRAW_U: u32 = 0x7e001077;
const MASK_KSLRAW_U: u32 = 0xfe00707f;
const MATCH_KSTAS16: u32 = 0xc4002077;
const MASK_KSTAS16: u32 = 0xfe00707f;
const MATCH_KSTAS32: u32 = 0xc0002077;
const MASK_KSTAS32: u32 = 0xfe00707f;
const MATCH_KSTSA16: u32 = 0xc6002077;
const MASK_KSTSA16: u32 = 0xfe00707f;
const MATCH_KSTSA32: u32 = 0xc2002077;
const MASK_KSTSA32: u32 = 0xfe00707f;
const MATCH_KSUB16: u32 = 0x12000077;
const MASK_KSUB16: u32 = 0xfe00707f;
const MATCH_KSUB32: u32 = 0x12002077;
const MASK_KSUB32: u32 = 0xfe00707f;
const MATCH_KSUB64: u32 = 0x92001077;
const MASK_KSUB64: u32 = 0xfe00707f;
const MATCH_KSUB8: u32 = 0x1a000077;
const MASK_KSUB8: u32 = 0xfe00707f;
const MATCH_KSUBH: u32 = 0x6001077;
const MASK_KSUBH: u32 = 0xfe00707f;
const MATCH_KSUBW: u32 = 0x2001077;
const MASK_KSUBW: u32 = 0xfe00707f;
const MATCH_KWMMUL: u32 = 0x62001077;
const MASK_KWMMUL: u32 = 0xfe00707f;
const MATCH_KWMMUL_U: u32 = 0x72001077;
const MASK_KWMMUL_U: u32 = 0xfe00707f;
const MATCH_LB: u32 = 0x3;
const MASK_LB: u32 = 0x707f;
const MATCH_LB_AQ: u32 = 0x3400002f;
const MASK_LB_AQ: u32 = 0xfdf0707f;
const MATCH_LBU: u32 = 0x4003;
const MASK_LBU: u32 = 0x707f;
const MATCH_LD: u32 = 0x3003;
const MASK_LD: u32 = 0x707f;
const MATCH_LD_AQ: u32 = 0x3400302f;
const MASK_LD_AQ: u32 = 0xfdf0707f;
const MATCH_LDU: u32 = 0x7003;
const MASK_LDU: u32 = 0x707f;
const MATCH_LH: u32 = 0x1003;
const MASK_LH: u32 = 0x707f;
const MATCH_LH_AQ: u32 = 0x3400102f;
const MASK_LH_AQ: u32 = 0xfdf0707f;
const MATCH_LHU: u32 = 0x5003;
const MASK_LHU: u32 = 0x707f;
const MATCH_LPAD: u32 = 0x17;
const MASK_LPAD: u32 = 0xfff;
const MATCH_LQ: u32 = 0x300f;
const MASK_LQ: u32 = 0x707f;
const MATCH_LR_D: u32 = 0x1000302f;
const MASK_LR_D: u32 = 0xf9f0707f;
const MATCH_LR_W: u32 = 0x1000202f;
const MASK_LR_W: u32 = 0xf9f0707f;
const MATCH_LUI: u32 = 0x37;
const MASK_LUI: u32 = 0x7f;
const MATCH_LW: u32 = 0x2003;
const MASK_LW: u32 = 0x707f;
const MATCH_LW_AQ: u32 = 0x3400202f;
const MASK_LW_AQ: u32 = 0xfdf0707f;
const MATCH_LWU: u32 = 0x6003;
const MASK_LWU: u32 = 0x707f;
const MATCH_MADDR32: u32 = 0xc4001077;
const MASK_MADDR32: u32 = 0xfe00707f;
const MATCH_MAX: u32 = 0xa006033;
const MASK_MAX: u32 = 0xfe00707f;
const MATCH_MAXU: u32 = 0xa007033;
const MASK_MAXU: u32 = 0xfe00707f;
const MATCH_MIN: u32 = 0xa004033;
const MASK_MIN: u32 = 0xfe00707f;
const MATCH_MINU: u32 = 0xa005033;
const MASK_MINU: u32 = 0xfe00707f;
const MATCH_MNRET: u32 = 0x70200073;
const MASK_MNRET: u32 = 0xffffffff;
const MATCH_MOP_R_0: u32 = 0x81c04073;
const MASK_MOP_R_0: u32 = 0xfff0707f;
const MATCH_MOP_R_1: u32 = 0x81d04073;
const MASK_MOP_R_1: u32 = 0xfff0707f;
const MATCH_MOP_R_10: u32 = 0x89e04073;
const MASK_MOP_R_10: u32 = 0xfff0707f;
const MATCH_MOP_R_11: u32 = 0x89f04073;
const MASK_MOP_R_11: u32 = 0xfff0707f;
const MATCH_MOP_R_12: u32 = 0x8dc04073;
const MASK_MOP_R_12: u32 = 0xfff0707f;
const MATCH_MOP_R_13: u32 = 0x8dd04073;
const MASK_MOP_R_13: u32 = 0xfff0707f;
const MATCH_MOP_R_14: u32 = 0x8de04073;
const MASK_MOP_R_14: u32 = 0xfff0707f;
const MATCH_MOP_R_15: u32 = 0x8df04073;
const MASK_MOP_R_15: u32 = 0xfff0707f;
const MATCH_MOP_R_16: u32 = 0xc1c04073;
const MASK_MOP_R_16: u32 = 0xfff0707f;
const MATCH_MOP_R_17: u32 = 0xc1d04073;
const MASK_MOP_R_17: u32 = 0xfff0707f;
const MATCH_MOP_R_18: u32 = 0xc1e04073;
const MASK_MOP_R_18: u32 = 0xfff0707f;
const MATCH_MOP_R_19: u32 = 0xc1f04073;
const MASK_MOP_R_19: u32 = 0xfff0707f;
const MATCH_MOP_R_2: u32 = 0x81e04073;
const MASK_MOP_R_2: u32 = 0xfff0707f;
const MATCH_MOP_R_20: u32 = 0xc5c04073;
const MASK_MOP_R_20: u32 = 0xfff0707f;
const MATCH_MOP_R_21: u32 = 0xc5d04073;
const MASK_MOP_R_21: u32 = 0xfff0707f;
const MATCH_MOP_R_22: u32 = 0xc5e04073;
const MASK_MOP_R_22: u32 = 0xfff0707f;
const MATCH_MOP_R_23: u32 = 0xc5f04073;
const MASK_MOP_R_23: u32 = 0xfff0707f;
const MATCH_MOP_R_24: u32 = 0xc9c04073;
const MASK_MOP_R_24: u32 = 0xfff0707f;
const MATCH_MOP_R_25: u32 = 0xc9d04073;
const MASK_MOP_R_25: u32 = 0xfff0707f;
const MATCH_MOP_R_26: u32 = 0xc9e04073;
const MASK_MOP_R_26: u32 = 0xfff0707f;
const MATCH_MOP_R_27: u32 = 0xc9f04073;
const MASK_MOP_R_27: u32 = 0xfff0707f;
const MATCH_MOP_R_28: u32 = 0xcdc04073;
const MASK_MOP_R_28: u32 = 0xfff0707f;
const MATCH_MOP_R_29: u32 = 0xcdd04073;
const MASK_MOP_R_29: u32 = 0xfff0707f;
const MATCH_MOP_R_3: u32 = 0x81f04073;
const MASK_MOP_R_3: u32 = 0xfff0707f;
const MATCH_MOP_R_30: u32 = 0xcde04073;
const MASK_MOP_R_30: u32 = 0xfff0707f;
const MATCH_MOP_R_31: u32 = 0xcdf04073;
const MASK_MOP_R_31: u32 = 0xfff0707f;
const MATCH_MOP_R_4: u32 = 0x85c04073;
const MASK_MOP_R_4: u32 = 0xfff0707f;
const MATCH_MOP_R_5: u32 = 0x85d04073;
const MASK_MOP_R_5: u32 = 0xfff0707f;
const MATCH_MOP_R_6: u32 = 0x85e04073;
const MASK_MOP_R_6: u32 = 0xfff0707f;
const MATCH_MOP_R_7: u32 = 0x85f04073;
const MASK_MOP_R_7: u32 = 0xfff0707f;
const MATCH_MOP_R_8: u32 = 0x89c04073;
const MASK_MOP_R_8: u32 = 0xfff0707f;
const MATCH_MOP_R_9: u32 = 0x89d04073;
const MASK_MOP_R_9: u32 = 0xfff0707f;
const MATCH_MOP_R_N: u32 = 0x81c04073;
const MASK_MOP_R_N: u32 = 0xb3c0707f;
const MATCH_MOP_RR_0: u32 = 0x82004073;
const MASK_MOP_RR_0: u32 = 0xfe00707f;
const MATCH_MOP_RR_1: u32 = 0x86004073;
const MASK_MOP_RR_1: u32 = 0xfe00707f;
const MATCH_MOP_RR_2: u32 = 0x8a004073;
const MASK_MOP_RR_2: u32 = 0xfe00707f;
const MATCH_MOP_RR_3: u32 = 0x8e004073;
const MASK_MOP_RR_3: u32 = 0xfe00707f;
const MATCH_MOP_RR_4: u32 = 0xc2004073;
const MASK_MOP_RR_4: u32 = 0xfe00707f;
const MATCH_MOP_RR_5: u32 = 0xc6004073;
const MASK_MOP_RR_5: u32 = 0xfe00707f;
const MATCH_MOP_RR_6: u32 = 0xca004073;
const MASK_MOP_RR_6: u32 = 0xfe00707f;
const MATCH_MOP_RR_7: u32 = 0xce004073;
const MASK_MOP_RR_7: u32 = 0xfe00707f;
const MATCH_MOP_RR_N: u32 = 0x82004073;
const MASK_MOP_RR_N: u32 = 0xb200707f;
const MATCH_MRET: u32 = 0x30200073;
const MASK_MRET: u32 = 0xffffffff;
const MATCH_MSUBR32: u32 = 0xc6001077;
const MASK_MSUBR32: u32 = 0xfe00707f;
const MATCH_MUL: u32 = 0x2000033;
const MASK_MUL: u32 = 0xfe00707f;
const MATCH_MULH: u32 = 0x2001033;
const MASK_MULH: u32 = 0xfe00707f;
const MATCH_MULHSU: u32 = 0x2002033;
const MASK_MULHSU: u32 = 0xfe00707f;
const MATCH_MULHU: u32 = 0x2003033;
const MASK_MULHU: u32 = 0xfe00707f;
const MATCH_MULR64: u32 = 0xf0001077;
const MASK_MULR64: u32 = 0xfe00707f;
const MATCH_MULSR64: u32 = 0xe0001077;
const MASK_MULSR64: u32 = 0xfe00707f;
const MATCH_MULW: u32 = 0x200003b;
const MASK_MULW: u32 = 0xfe00707f;
const MATCH_NTL_ALL: u32 = 0x500033;
const MASK_NTL_ALL: u32 = 0xffffffff;
const MATCH_NTL_P1: u32 = 0x200033;
const MASK_NTL_P1: u32 = 0xffffffff;
const MATCH_NTL_PALL: u32 = 0x300033;
const MASK_NTL_PALL: u32 = 0xffffffff;
const MATCH_NTL_S1: u32 = 0x400033;
const MASK_NTL_S1: u32 = 0xffffffff;
const MATCH_OR: u32 = 0x6033;
const MASK_OR: u32 = 0xfe00707f;
const MATCH_ORC_B: u32 = 0x28705013;
const MASK_ORC_B: u32 = 0xfff0707f;
const MATCH_ORI: u32 = 0x6013;
const MASK_ORI: u32 = 0x707f;
const MATCH_ORN: u32 = 0x40006033;
const MASK_ORN: u32 = 0xfe00707f;
const MATCH_PACK: u32 = 0x8004033;
const MASK_PACK: u32 = 0xfe00707f;
const MATCH_PACKH: u32 = 0x8007033;
const MASK_PACKH: u32 = 0xfe00707f;
const MATCH_PACKU: u32 = 0x48004033;
const MASK_PACKU: u32 = 0xfe00707f;
const MATCH_PACKUW: u32 = 0x4800403b;
const MASK_PACKUW: u32 = 0xfe00707f;
const MATCH_PACKW: u32 = 0x800403b;
const MASK_PACKW: u32 = 0xfe00707f;
const MATCH_PAUSE: u32 = 0x100000f;
const MASK_PAUSE: u32 = 0xffffffff;
const MATCH_PBSAD: u32 = 0xfc000077;
const MASK_PBSAD: u32 = 0xfe00707f;
const MATCH_PBSADA: u32 = 0xfe000077;
const MASK_PBSADA: u32 = 0xfe00707f;
const MATCH_PKBB16: u32 = 0xe001077;
const MASK_PKBB16: u32 = 0xfe00707f;
const MATCH_PKBT16: u32 = 0x1e001077;
const MASK_PKBT16: u32 = 0xfe00707f;
const MATCH_PKBT32: u32 = 0x1e002077;
const MASK_PKBT32: u32 = 0xfe00707f;
const MATCH_PKTB16: u32 = 0x3e001077;
const MASK_PKTB16: u32 = 0xfe00707f;
const MATCH_PKTB32: u32 = 0x3e002077;
const MASK_PKTB32: u32 = 0xfe00707f;
const MATCH_PKTT16: u32 = 0x2e001077;
const MASK_PKTT16: u32 = 0xfe00707f;
const MATCH_PREFETCH_I: u32 = 0x6013;
const MASK_PREFETCH_I: u32 = 0x1f07fff;
const MATCH_PREFETCH_R: u32 = 0x106013;
const MASK_PREFETCH_R: u32 = 0x1f07fff;
const MATCH_PREFETCH_W: u32 = 0x306013;
const MASK_PREFETCH_W: u32 = 0x1f07fff;
const MATCH_RADD16: u32 = 0x77;
const MASK_RADD16: u32 = 0xfe00707f;
const MATCH_RADD32: u32 = 0x2077;
const MASK_RADD32: u32 = 0xfe00707f;
const MATCH_RADD64: u32 = 0x80001077;
const MASK_RADD64: u32 = 0xfe00707f;
const MATCH_RADD8: u32 = 0x8000077;
const MASK_RADD8: u32 = 0xfe00707f;
const MATCH_RADDW: u32 = 0x20001077;
const MASK_RADDW: u32 = 0xfe00707f;
const MATCH_RCRAS16: u32 = 0x4000077;
const MASK_RCRAS16: u32 = 0xfe00707f;
const MATCH_RCRAS32: u32 = 0x4002077;
const MASK_RCRAS32: u32 = 0xfe00707f;
const MATCH_RCRSA16: u32 = 0x6000077;
const MASK_RCRSA16: u32 = 0xfe00707f;
const MATCH_RCRSA32: u32 = 0x6002077;
const MASK_RCRSA32: u32 = 0xfe00707f;
const MATCH_RDCYCLE: u32 = 0xc0002073;
const MASK_RDCYCLE: u32 = 0xfffff07f;
const MATCH_RDCYCLEH: u32 = 0xc8002073;
const MASK_RDCYCLEH: u32 = 0xfffff07f;
const MATCH_RDINSTRET: u32 = 0xc0202073;
const MASK_RDINSTRET: u32 = 0xfffff07f;
const MATCH_RDINSTRETH: u32 = 0xc8202073;
const MASK_RDINSTRETH: u32 = 0xfffff07f;
const MATCH_RDOV: u32 = 0x902073;
const MASK_RDOV: u32 = 0xfffff07f;
const MATCH_RDTIME: u32 = 0xc0102073;
const MASK_RDTIME: u32 = 0xfffff07f;
const MATCH_RDTIMEH: u32 = 0xc8102073;
const MASK_RDTIMEH: u32 = 0xfffff07f;
const MATCH_REM: u32 = 0x2006033;
const MASK_REM: u32 = 0xfe00707f;
const MATCH_REMU: u32 = 0x2007033;
const MASK_REMU: u32 = 0xfe00707f;
const MATCH_REMUW: u32 = 0x200703b;
const MASK_REMUW: u32 = 0xfe00707f;
const MATCH_REMW: u32 = 0x200603b;
const MASK_REMW: u32 = 0xfe00707f;
const MATCH_REV: u32 = 0x6bf05013;
const MASK_REV: u32 = 0xfff0707f;
const MATCH_REV8: u32 = 0x6b805013;
const MASK_REV8: u32 = 0xfff0707f;
const MATCH_REV8_H: u32 = 0x68805013;
const MASK_REV8_H: u32 = 0xfff0707f;
const MATCH_REV8_RV32: u32 = 0x69805013;
const MASK_REV8_RV32: u32 = 0xfff0707f;
const MATCH_ROL: u32 = 0x60001033;
const MASK_ROL: u32 = 0xfe00707f;
const MATCH_ROLW: u32 = 0x6000103b;
const MASK_ROLW: u32 = 0xfe00707f;
const MATCH_ROR: u32 = 0x60005033;
const MASK_ROR: u32 = 0xfe00707f;
const MATCH_RORI: u32 = 0x60005013;
const MASK_RORI: u32 = 0xfc00707f;
const MATCH_RORI_RV32: u32 = 0x60005013;
const MASK_RORI_RV32: u32 = 0xfe00707f;
const MATCH_RORIW: u32 = 0x6000501b;
const MASK_RORIW: u32 = 0xfe00707f;
const MATCH_RORW: u32 = 0x6000503b;
const MASK_RORW: u32 = 0xfe00707f;
const MATCH_RSTAS16: u32 = 0xb4002077;
const MASK_RSTAS16: u32 = 0xfe00707f;
const MATCH_RSTAS32: u32 = 0xb0002077;
const MASK_RSTAS32: u32 = 0xfe00707f;
const MATCH_RSTSA16: u32 = 0xb6002077;
const MASK_RSTSA16: u32 = 0xfe00707f;
const MATCH_RSTSA32: u32 = 0xb2002077;
const MASK_RSTSA32: u32 = 0xfe00707f;
const MATCH_RSUB16: u32 = 0x2000077;
const MASK_RSUB16: u32 = 0xfe00707f;
const MATCH_RSUB32: u32 = 0x2002077;
const MASK_RSUB32: u32 = 0xfe00707f;
const MATCH_RSUB64: u32 = 0x82001077;
const MASK_RSUB64: u32 = 0xfe00707f;
const MATCH_RSUB8: u32 = 0xa000077;
const MASK_RSUB8: u32 = 0xfe00707f;
const MATCH_RSUBW: u32 = 0x22001077;
const MASK_RSUBW: u32 = 0xfe00707f;
const MATCH_SB: u32 = 0x23;
const MASK_SB: u32 = 0x707f;
const MATCH_SB_RL: u32 = 0x3a00002f;
const MASK_SB_RL: u32 = 0xfa007fff;
const MATCH_SBREAK: u32 = 0x100073;
const MASK_SBREAK: u32 = 0xffffffff;
const MATCH_SC_D: u32 = 0x1800302f;
const MASK_SC_D: u32 = 0xf800707f;
const MATCH_SC_W: u32 = 0x1800202f;
const MASK_SC_W: u32 = 0xf800707f;
const MATCH_SCALL: u32 = 0x73;
const MASK_SCALL: u32 = 0xffffffff;
const MATCH_SCLIP16: u32 = 0x84000077;
const MASK_SCLIP16: u32 = 0xff00707f;
const MATCH_SCLIP32: u32 = 0xe4000077;
const MASK_SCLIP32: u32 = 0xfe00707f;
const MATCH_SCLIP8: u32 = 0x8c000077;
const MASK_SCLIP8: u32 = 0xff80707f;
const MATCH_SCMPLE16: u32 = 0x1c000077;
const MASK_SCMPLE16: u32 = 0xfe00707f;
const MATCH_SCMPLE8: u32 = 0x1e000077;
const MASK_SCMPLE8: u32 = 0xfe00707f;
const MATCH_SCMPLT16: u32 = 0xc000077;
const MASK_SCMPLT16: u32 = 0xfe00707f;
const MATCH_SCMPLT8: u32 = 0xe000077;
const MASK_SCMPLT8: u32 = 0xfe00707f;
const MATCH_SD: u32 = 0x3023;
const MASK_SD: u32 = 0x707f;
const MATCH_SD_RL: u32 = 0x3a00302f;
const MASK_SD_RL: u32 = 0xfa007fff;
const MATCH_SEXT_B: u32 = 0x60401013;
const MASK_SEXT_B: u32 = 0xfff0707f;
const MATCH_SEXT_H: u32 = 0x60501013;
const MASK_SEXT_H: u32 = 0xfff0707f;
const MATCH_SFENCE_INVAL_IR: u32 = 0x18100073;
const MASK_SFENCE_INVAL_IR: u32 = 0xffffffff;
const MATCH_SFENCE_VMA: u32 = 0x12000073;
const MASK_SFENCE_VMA: u32 = 0xfe007fff;
const MATCH_SFENCE_W_INVAL: u32 = 0x18000073;
const MASK_SFENCE_W_INVAL: u32 = 0xffffffff;
const MATCH_SH: u32 = 0x1023;
const MASK_SH: u32 = 0x707f;
const MATCH_SH1ADD: u32 = 0x20002033;
const MASK_SH1ADD: u32 = 0xfe00707f;
const MATCH_SH1ADD_UW: u32 = 0x2000203b;
const MASK_SH1ADD_UW: u32 = 0xfe00707f;
const MATCH_SH2ADD: u32 = 0x20004033;
const MASK_SH2ADD: u32 = 0xfe00707f;
const MATCH_SH2ADD_UW: u32 = 0x2000403b;
const MASK_SH2ADD_UW: u32 = 0xfe00707f;
const MATCH_SH3ADD: u32 = 0x20006033;
const MASK_SH3ADD: u32 = 0xfe00707f;
const MATCH_SH3ADD_UW: u32 = 0x2000603b;
const MASK_SH3ADD_UW: u32 = 0xfe00707f;
const MATCH_SH_RL: u32 = 0x3a00102f;
const MASK_SH_RL: u32 = 0xfa007fff;
const MATCH_SHA256SIG0: u32 = 0x10201013;
const MASK_SHA256SIG0: u32 = 0xfff0707f;
const MATCH_SHA256SIG1: u32 = 0x10301013;
const MASK_SHA256SIG1: u32 = 0xfff0707f;
const MATCH_SHA256SUM0: u32 = 0x10001013;
const MASK_SHA256SUM0: u32 = 0xfff0707f;
const MATCH_SHA256SUM1: u32 = 0x10101013;
const MASK_SHA256SUM1: u32 = 0xfff0707f;
const MATCH_SHA512SIG0: u32 = 0x10601013;
const MASK_SHA512SIG0: u32 = 0xfff0707f;
const MATCH_SHA512SIG0H: u32 = 0x5c000033;
const MASK_SHA512SIG0H: u32 = 0xfe00707f;
const MATCH_SHA512SIG0L: u32 = 0x54000033;
const MASK_SHA512SIG0L: u32 = 0xfe00707f;
const MATCH_SHA512SIG1: u32 = 0x10701013;
const MASK_SHA512SIG1: u32 = 0xfff0707f;
const MATCH_SHA512SIG1H: u32 = 0x5e000033;
const MASK_SHA512SIG1H: u32 = 0xfe00707f;
const MATCH_SHA512SIG1L: u32 = 0x56000033;
const MASK_SHA512SIG1L: u32 = 0xfe00707f;
const MATCH_SHA512SUM0: u32 = 0x10401013;
const MASK_SHA512SUM0: u32 = 0xfff0707f;
const MATCH_SHA512SUM0R: u32 = 0x50000033;
const MASK_SHA512SUM0R: u32 = 0xfe00707f;
const MATCH_SHA512SUM1: u32 = 0x10501013;
const MASK_SHA512SUM1: u32 = 0xfff0707f;
const MATCH_SHA512SUM1R: u32 = 0x52000033;
const MASK_SHA512SUM1R: u32 = 0xfe00707f;
const MATCH_SHFL: u32 = 0x8001033;
const MASK_SHFL: u32 = 0xfe00707f;
const MATCH_SHFLI: u32 = 0x8001013;
const MASK_SHFLI: u32 = 0xfe00707f;
const MATCH_SHFLW: u32 = 0x800103b;
const MASK_SHFLW: u32 = 0xfe00707f;
const MATCH_SINVAL_VMA: u32 = 0x16000073;
const MASK_SINVAL_VMA: u32 = 0xfe007fff;
const MATCH_SLL: u32 = 0x1033;
const MASK_SLL: u32 = 0xfe00707f;
const MATCH_SLL16: u32 = 0x54000077;
const MASK_SLL16: u32 = 0xfe00707f;
const MATCH_SLL32: u32 = 0x54002077;
const MASK_SLL32: u32 = 0xfe00707f;
const MATCH_SLL8: u32 = 0x5c000077;
const MASK_SLL8: u32 = 0xfe00707f;
const MATCH_SLLD: u32 = 0x107b;
const MASK_SLLD: u32 = 0xfe00707f;
const MATCH_SLLI: u32 = 0x1013;
const MASK_SLLI: u32 = 0xfc00707f;
const MATCH_SLLI16: u32 = 0x74000077;
const MASK_SLLI16: u32 = 0xff00707f;
const MATCH_SLLI32: u32 = 0x74002077;
const MASK_SLLI32: u32 = 0xfe00707f;
const MATCH_SLLI8: u32 = 0x7c000077;
const MASK_SLLI8: u32 = 0xff80707f;
const MATCH_SLLI_RV128: u32 = 0x1013;
const MASK_SLLI_RV128: u32 = 0xf800707f;
const MATCH_SLLI_RV32: u32 = 0x1013;
const MASK_SLLI_RV32: u32 = 0xfe00707f;
const MATCH_SLLI_UW: u32 = 0x800101b;
const MASK_SLLI_UW: u32 = 0xfc00707f;
const MATCH_SLLID: u32 = 0x105b;
const MASK_SLLID: u32 = 0xfc00707f;
const MATCH_SLLIW: u32 = 0x101b;
const MASK_SLLIW: u32 = 0xfe00707f;
const MATCH_SLLW: u32 = 0x103b;
const MASK_SLLW: u32 = 0xfe00707f;
const MATCH_SLO: u32 = 0x20001033;
const MASK_SLO: u32 = 0xfe00707f;
const MATCH_SLOI: u32 = 0x20001013;
const MASK_SLOI: u32 = 0xfc00707f;
const MATCH_SLOIW: u32 = 0x2000101b;
const MASK_SLOIW: u32 = 0xfe00707f;
const MATCH_SLOW: u32 = 0x2000103b;
const MASK_SLOW: u32 = 0xfe00707f;
const MATCH_SLT: u32 = 0x2033;
const MASK_SLT: u32 = 0xfe00707f;
const MATCH_SLTI: u32 = 0x2013;
const MASK_SLTI: u32 = 0x707f;
const MATCH_SLTIU: u32 = 0x3013;
const MASK_SLTIU: u32 = 0x707f;
const MATCH_SLTU: u32 = 0x3033;
const MASK_SLTU: u32 = 0xfe00707f;
const MATCH_SM3P0: u32 = 0x10801013;
const MASK_SM3P0: u32 = 0xfff0707f;
const MATCH_SM3P1: u32 = 0x10901013;
const MASK_SM3P1: u32 = 0xfff0707f;
const MATCH_SM4ED: u32 = 0x30000033;
const MASK_SM4ED: u32 = 0x3e00707f;
const MATCH_SM4KS: u32 = 0x34000033;
const MASK_SM4KS: u32 = 0x3e00707f;
const MATCH_SMAL: u32 = 0x5e001077;
const MASK_SMAL: u32 = 0xfe00707f;
const MATCH_SMALBB: u32 = 0x88001077;
const MASK_SMALBB: u32 = 0xfe00707f;
const MATCH_SMALBT: u32 = 0x98001077;
const MASK_SMALBT: u32 = 0xfe00707f;
const MATCH_SMALDA: u32 = 0x8c001077;
const MASK_SMALDA: u32 = 0xfe00707f;
const MATCH_SMALDRS: u32 = 0x9a001077;
const MASK_SMALDRS: u32 = 0xfe00707f;
const MATCH_SMALDS: u32 = 0x8a001077;
const MASK_SMALDS: u32 = 0xfe00707f;
const MATCH_SMALTT: u32 = 0xa8001077;
const MASK_SMALTT: u32 = 0xfe00707f;
const MATCH_SMALXDA: u32 = 0x9c001077;
const MASK_SMALXDA: u32 = 0xfe00707f;
const MATCH_SMALXDS: u32 = 0xaa001077;
const MASK_SMALXDS: u32 = 0xfe00707f;
const MATCH_SMAQA: u32 = 0xc8000077;
const MASK_SMAQA: u32 = 0xfe00707f;
const MATCH_SMAQA_SU: u32 = 0xca000077;
const MASK_SMAQA_SU: u32 = 0xfe00707f;
const MATCH_SMAR64: u32 = 0x84001077;
const MASK_SMAR64: u32 = 0xfe00707f;
const MATCH_SMAX16: u32 = 0x82000077;
const MASK_SMAX16: u32 = 0xfe00707f;
const MATCH_SMAX32: u32 = 0x92002077;
const MASK_SMAX32: u32 = 0xfe00707f;
const MATCH_SMAX8: u32 = 0x8a000077;
const MASK_SMAX8: u32 = 0xfe00707f;
const MATCH_SMBB16: u32 = 0x8001077;
const MASK_SMBB16: u32 = 0xfe00707f;
const MATCH_SMBT16: u32 = 0x18001077;
const MASK_SMBT16: u32 = 0xfe00707f;
const MATCH_SMBT32: u32 = 0x18002077;
const MASK_SMBT32: u32 = 0xfe00707f;
const MATCH_SMDRS: u32 = 0x68001077;
const MASK_SMDRS: u32 = 0xfe00707f;
const MATCH_SMDRS32: u32 = 0x68002077;
const MASK_SMDRS32: u32 = 0xfe00707f;
const MATCH_SMDS: u32 = 0x58001077;
const MASK_SMDS: u32 = 0xfe00707f;
const MATCH_SMDS32: u32 = 0x58002077;
const MASK_SMDS32: u32 = 0xfe00707f;
const MATCH_SMIN16: u32 = 0x80000077;
const MASK_SMIN16: u32 = 0xfe00707f;
const MATCH_SMIN32: u32 = 0x90002077;
const MASK_SMIN32: u32 = 0xfe00707f;
const MATCH_SMIN8: u32 = 0x88000077;
const MASK_SMIN8: u32 = 0xfe00707f;
const MATCH_SMMUL: u32 = 0x40001077;
const MASK_SMMUL: u32 = 0xfe00707f;
const MATCH_SMMUL_U: u32 = 0x50001077;
const MASK_SMMUL_U: u32 = 0xfe00707f;
const MATCH_SMMWB: u32 = 0x44001077;
const MASK_SMMWB: u32 = 0xfe00707f;
const MATCH_SMMWB_U: u32 = 0x54001077;
const MASK_SMMWB_U: u32 = 0xfe00707f;
const MATCH_SMMWT: u32 = 0x64001077;
const MASK_SMMWT: u32 = 0xfe00707f;
const MATCH_SMMWT_U: u32 = 0x74001077;
const MASK_SMMWT_U: u32 = 0xfe00707f;
const MATCH_SMSLDA: u32 = 0xac001077;
const MASK_SMSLDA: u32 = 0xfe00707f;
const MATCH_SMSLXDA: u32 = 0xbc001077;
const MASK_SMSLXDA: u32 = 0xfe00707f;
const MATCH_SMSR64: u32 = 0x86001077;
const MASK_SMSR64: u32 = 0xfe00707f;
const MATCH_SMTT16: u32 = 0x28001077;
const MASK_SMTT16: u32 = 0xfe00707f;
const MATCH_SMTT32: u32 = 0x28002077;
const MASK_SMTT32: u32 = 0xfe00707f;
const MATCH_SMUL16: u32 = 0xa0000077;
const MASK_SMUL16: u32 = 0xfe00707f;
const MATCH_SMUL8: u32 = 0xa8000077;
const MASK_SMUL8: u32 = 0xfe00707f;
const MATCH_SMULX16: u32 = 0xa2000077;
const MASK_SMULX16: u32 = 0xfe00707f;
const MATCH_SMULX8: u32 = 0xaa000077;
const MASK_SMULX8: u32 = 0xfe00707f;
const MATCH_SMXDS: u32 = 0x78001077;
const MASK_SMXDS: u32 = 0xfe00707f;
const MATCH_SMXDS32: u32 = 0x78002077;
const MASK_SMXDS32: u32 = 0xfe00707f;
const MATCH_SQ: u32 = 0x4023;
const MASK_SQ: u32 = 0x707f;
const MATCH_SRA: u32 = 0x40005033;
const MASK_SRA: u32 = 0xfe00707f;
const MATCH_SRA16: u32 = 0x50000077;
const MASK_SRA16: u32 = 0xfe00707f;
const MATCH_SRA16_U: u32 = 0x60000077;
const MASK_SRA16_U: u32 = 0xfe00707f;
const MATCH_SRA32: u32 = 0x50002077;
const MASK_SRA32: u32 = 0xfe00707f;
const MATCH_SRA32_U: u32 = 0x60002077;
const MASK_SRA32_U: u32 = 0xfe00707f;
const MATCH_SRA8: u32 = 0x58000077;
const MASK_SRA8: u32 = 0xfe00707f;
const MATCH_SRA8_U: u32 = 0x68000077;
const MASK_SRA8_U: u32 = 0xfe00707f;
const MATCH_SRA_U: u32 = 0x24001077;
const MASK_SRA_U: u32 = 0xfe00707f;
const MATCH_SRAD: u32 = 0x4000507b;
const MASK_SRAD: u32 = 0xfe00707f;
const MATCH_SRAI: u32 = 0x40005013;
const MASK_SRAI: u32 = 0xfc00707f;
const MATCH_SRAI16: u32 = 0x70000077;
const MASK_SRAI16: u32 = 0xff00707f;
const MATCH_SRAI16_U: u32 = 0x71000077;
const MASK_SRAI16_U: u32 = 0xff00707f;
const MATCH_SRAI32: u32 = 0x70002077;
const MASK_SRAI32: u32 = 0xfe00707f;
const MATCH_SRAI32_U: u32 = 0x80002077;
const MASK_SRAI32_U: u32 = 0xfe00707f;
const MATCH_SRAI8: u32 = 0x78000077;
const MASK_SRAI8: u32 = 0xff80707f;
const MATCH_SRAI8_U: u32 = 0x78800077;
const MASK_SRAI8_U: u32 = 0xff80707f;
const MATCH_SRAI_RV128: u32 = 0x40005013;
const MASK_SRAI_RV128: u32 = 0xf800707f;
const MATCH_SRAI_RV32: u32 = 0x40005013;
const MASK_SRAI_RV32: u32 = 0xfe00707f;
const MATCH_SRAI_U: u32 = 0xd4001077;
const MASK_SRAI_U: u32 = 0xfc00707f;
const MATCH_SRAID: u32 = 0x4000505b;
const MASK_SRAID: u32 = 0xfc00707f;
const MATCH_SRAIW: u32 = 0x4000501b;
const MASK_SRAIW: u32 = 0xfe00707f;
const MATCH_SRAIW_U: u32 = 0x34001077;
const MASK_SRAIW_U: u32 = 0xfe00707f;
const MATCH_SRAW: u32 = 0x4000503b;
const MASK_SRAW: u32 = 0xfe00707f;
const MATCH_SRET: u32 = 0x10200073;
const MASK_SRET: u32 = 0xffffffff;
const MATCH_SRL: u32 = 0x5033;
const MASK_SRL: u32 = 0xfe00707f;
const MATCH_SRL16: u32 = 0x52000077;
const MASK_SRL16: u32 = 0xfe00707f;
const MATCH_SRL16_U: u32 = 0x62000077;
const MASK_SRL16_U: u32 = 0xfe00707f;
const MATCH_SRL32: u32 = 0x52002077;
const MASK_SRL32: u32 = 0xfe00707f;
const MATCH_SRL32_U: u32 = 0x62002077;
const MASK_SRL32_U: u32 = 0xfe00707f;
const MATCH_SRL8: u32 = 0x5a000077;
const MASK_SRL8: u32 = 0xfe00707f;
const MATCH_SRL8_U: u32 = 0x6a000077;
const MASK_SRL8_U: u32 = 0xfe00707f;
const MATCH_SRLD: u32 = 0x507b;
const MASK_SRLD: u32 = 0xfe00707f;
const MATCH_SRLI: u32 = 0x5013;
const MASK_SRLI: u32 = 0xfc00707f;
const MATCH_SRLI16: u32 = 0x72000077;
const MASK_SRLI16: u32 = 0xff00707f;
const MATCH_SRLI16_U: u32 = 0x73000077;
const MASK_SRLI16_U: u32 = 0xff00707f;
const MATCH_SRLI32: u32 = 0x72002077;
const MASK_SRLI32: u32 = 0xfe00707f;
const MATCH_SRLI32_U: u32 = 0x82002077;
const MASK_SRLI32_U: u32 = 0xfe00707f;
const MATCH_SRLI8: u32 = 0x7a000077;
const MASK_SRLI8: u32 = 0xff80707f;
const MATCH_SRLI8_U: u32 = 0x7a800077;
const MASK_SRLI8_U: u32 = 0xff80707f;
const MATCH_SRLI_RV128: u32 = 0x5013;
const MASK_SRLI_RV128: u32 = 0xf800707f;
const MATCH_SRLI_RV32: u32 = 0x5013;
const MASK_SRLI_RV32: u32 = 0xfe00707f;
const MATCH_SRLID: u32 = 0x505b;
const MASK_SRLID: u32 = 0xfc00707f;
const MATCH_SRLIW: u32 = 0x501b;
const MASK_SRLIW: u32 = 0xfe00707f;
const MATCH_SRLW: u32 = 0x503b;
const MASK_SRLW: u32 = 0xfe00707f;
const MATCH_SRO: u32 = 0x20005033;
const MASK_SRO: u32 = 0xfe00707f;
const MATCH_SROI: u32 = 0x20005013;
const MASK_SROI: u32 = 0xfc00707f;
const MATCH_SROIW: u32 = 0x2000501b;
const MASK_SROIW: u32 = 0xfe00707f;
const MATCH_SROW: u32 = 0x2000503b;
const MASK_SROW: u32 = 0xfe00707f;
const MATCH_SSAMOSWAP_D: u32 = 0x4800302f;
const MASK_SSAMOSWAP_D: u32 = 0xf800707f;
const MATCH_SSAMOSWAP_W: u32 = 0x4800202f;
const MASK_SSAMOSWAP_W: u32 = 0xf800707f;
const MATCH_SSPOPCHK_X1: u32 = 0xcdc0c073;
const MASK_SSPOPCHK_X1: u32 = 0xffffffff;
const MATCH_SSPOPCHK_X5: u32 = 0xcdc2c073;
const MASK_SSPOPCHK_X5: u32 = 0xffffffff;
const MATCH_SSPUSH_X1: u32 = 0xce104073;
const MASK_SSPUSH_X1: u32 = 0xffffffff;
const MATCH_SSPUSH_X5: u32 = 0xce504073;
const MASK_SSPUSH_X5: u32 = 0xffffffff;
const MATCH_SSRDP: u32 = 0xcdc04073;
const MASK_SSRDP: u32 = 0xfffff07f;
const MATCH_STAS16: u32 = 0xf4002077;
const MASK_STAS16: u32 = 0xfe00707f;
const MATCH_STAS32: u32 = 0xf0002077;
const MASK_STAS32: u32 = 0xfe00707f;
const MATCH_STSA16: u32 = 0xf6002077;
const MASK_STSA16: u32 = 0xfe00707f;
const MATCH_STSA32: u32 = 0xf2002077;
const MASK_STSA32: u32 = 0xfe00707f;
const MATCH_SUB: u32 = 0x40000033;
const MASK_SUB: u32 = 0xfe00707f;
const MATCH_SUB16: u32 = 0x42000077;
const MASK_SUB16: u32 = 0xfe00707f;
const MATCH_SUB32: u32 = 0x42002077;
const MASK_SUB32: u32 = 0xfe00707f;
const MATCH_SUB64: u32 = 0xc2001077;
const MASK_SUB64: u32 = 0xfe00707f;
const MATCH_SUB8: u32 = 0x4a000077;
const MASK_SUB8: u32 = 0xfe00707f;
const MATCH_SUBD: u32 = 0x4000007b;
const MASK_SUBD: u32 = 0xfe00707f;
const MATCH_SUBW: u32 = 0x4000003b;
const MASK_SUBW: u32 = 0xfe00707f;
const MATCH_SUNPKD810: u32 = 0xac800077;
const MASK_SUNPKD810: u32 = 0xfff0707f;
const MATCH_SUNPKD820: u32 = 0xac900077;
const MASK_SUNPKD820: u32 = 0xfff0707f;
const MATCH_SUNPKD830: u32 = 0xaca00077;
const MASK_SUNPKD830: u32 = 0xfff0707f;
const MATCH_SUNPKD831: u32 = 0xacb00077;
const MASK_SUNPKD831: u32 = 0xfff0707f;
const MATCH_SUNPKD832: u32 = 0xad300077;
const MASK_SUNPKD832: u32 = 0xfff0707f;
const MATCH_SW: u32 = 0x2023;
const MASK_SW: u32 = 0x707f;
const MATCH_SW_RL: u32 = 0x3a00202f;
const MASK_SW_RL: u32 = 0xfa007fff;
const MATCH_UCLIP16: u32 = 0x85000077;
const MASK_UCLIP16: u32 = 0xff00707f;
const MATCH_UCLIP32: u32 = 0xf4000077;
const MASK_UCLIP32: u32 = 0xfe00707f;
const MATCH_UCLIP8: u32 = 0x8d000077;
const MASK_UCLIP8: u32 = 0xff80707f;
const MATCH_UCMPLE16: u32 = 0x3c000077;
const MASK_UCMPLE16: u32 = 0xfe00707f;
const MATCH_UCMPLE8: u32 = 0x3e000077;
const MASK_UCMPLE8: u32 = 0xfe00707f;
const MATCH_UCMPLT16: u32 = 0x2c000077;
const MASK_UCMPLT16: u32 = 0xfe00707f;
const MATCH_UCMPLT8: u32 = 0x2e000077;
const MASK_UCMPLT8: u32 = 0xfe00707f;
const MATCH_UKADD16: u32 = 0x30000077;
const MASK_UKADD16: u32 = 0xfe00707f;
const MATCH_UKADD32: u32 = 0x30002077;
const MASK_UKADD32: u32 = 0xfe00707f;
const MATCH_UKADD64: u32 = 0xb0001077;
const MASK_UKADD64: u32 = 0xfe00707f;
const MATCH_UKADD8: u32 = 0x38000077;
const MASK_UKADD8: u32 = 0xfe00707f;
const MATCH_UKADDH: u32 = 0x14001077;
const MASK_UKADDH: u32 = 0xfe00707f;
const MATCH_UKADDW: u32 = 0x10001077;
const MASK_UKADDW: u32 = 0xfe00707f;
const MATCH_UKCRAS16: u32 = 0x34000077;
const MASK_UKCRAS16: u32 = 0xfe00707f;
const MATCH_UKCRAS32: u32 = 0x34002077;
const MASK_UKCRAS32: u32 = 0xfe00707f;
const MATCH_UKCRSA16: u32 = 0x36000077;
const MASK_UKCRSA16: u32 = 0xfe00707f;
const MATCH_UKCRSA32: u32 = 0x36002077;
const MASK_UKCRSA32: u32 = 0xfe00707f;
const MATCH_UKMAR64: u32 = 0xb4001077;
const MASK_UKMAR64: u32 = 0xfe00707f;
const MATCH_UKMSR64: u32 = 0xb6001077;
const MASK_UKMSR64: u32 = 0xfe00707f;
const MATCH_UKSTAS16: u32 = 0xe4002077;
const MASK_UKSTAS16: u32 = 0xfe00707f;
const MATCH_UKSTAS32: u32 = 0xe0002077;
const MASK_UKSTAS32: u32 = 0xfe00707f;
const MATCH_UKSTSA16: u32 = 0xe6002077;
const MASK_UKSTSA16: u32 = 0xfe00707f;
const MATCH_UKSTSA32: u32 = 0xe2002077;
const MASK_UKSTSA32: u32 = 0xfe00707f;
const MATCH_UKSUB16: u32 = 0x32000077;
const MASK_UKSUB16: u32 = 0xfe00707f;
const MATCH_UKSUB32: u32 = 0x32002077;
const MASK_UKSUB32: u32 = 0xfe00707f;
const MATCH_UKSUB64: u32 = 0xb2001077;
const MASK_UKSUB64: u32 = 0xfe00707f;
const MATCH_UKSUB8: u32 = 0x3a000077;
const MASK_UKSUB8: u32 = 0xfe00707f;
const MATCH_UKSUBH: u32 = 0x16001077;
const MASK_UKSUBH: u32 = 0xfe00707f;
const MATCH_UKSUBW: u32 = 0x12001077;
const MASK_UKSUBW: u32 = 0xfe00707f;
const MATCH_UMAQA: u32 = 0xcc000077;
const MASK_UMAQA: u32 = 0xfe00707f;
const MATCH_UMAR64: u32 = 0xa4001077;
const MASK_UMAR64: u32 = 0xfe00707f;
const MATCH_UMAX16: u32 = 0x92000077;
const MASK_UMAX16: u32 = 0xfe00707f;
const MATCH_UMAX32: u32 = 0xa2002077;
const MASK_UMAX32: u32 = 0xfe00707f;
const MATCH_UMAX8: u32 = 0x9a000077;
const MASK_UMAX8: u32 = 0xfe00707f;
const MATCH_UMIN16: u32 = 0x90000077;
const MASK_UMIN16: u32 = 0xfe00707f;
const MATCH_UMIN32: u32 = 0xa0002077;
const MASK_UMIN32: u32 = 0xfe00707f;
const MATCH_UMIN8: u32 = 0x98000077;
const MASK_UMIN8: u32 = 0xfe00707f;
const MATCH_UMSR64: u32 = 0xa6001077;
const MASK_UMSR64: u32 = 0xfe00707f;
const MATCH_UMUL16: u32 = 0xb0000077;
const MASK_UMUL16: u32 = 0xfe00707f;
const MATCH_UMUL8: u32 = 0xb8000077;
const MASK_UMUL8: u32 = 0xfe00707f;
const MATCH_UMULX16: u32 = 0xb2000077;
const MASK_UMULX16: u32 = 0xfe00707f;
const MATCH_UMULX8: u32 = 0xba000077;
const MASK_UMULX8: u32 = 0xfe00707f;
const MATCH_UNSHFL: u32 = 0x8005033;
const MASK_UNSHFL: u32 = 0xfe00707f;
const MATCH_UNSHFLI: u32 = 0x8005013;
const MASK_UNSHFLI: u32 = 0xfe00707f;
const MATCH_UNSHFLW: u32 = 0x800503b;
const MASK_UNSHFLW: u32 = 0xfe00707f;
const MATCH_UNZIP: u32 = 0x8f05013;
const MASK_UNZIP: u32 = 0xfff0707f;
const MATCH_UNZIP16: u32 = 0x9005013;
const MASK_UNZIP16: u32 = 0xfff0707f;
const MATCH_UNZIP8: u32 = 0x9805013;
const MASK_UNZIP8: u32 = 0xfff0707f;
const MATCH_URADD16: u32 = 0x20000077;
const MASK_URADD16: u32 = 0xfe00707f;
const MATCH_URADD32: u32 = 0x20002077;
const MASK_URADD32: u32 = 0xfe00707f;
const MATCH_URADD64: u32 = 0xa0001077;
const MASK_URADD64: u32 = 0xfe00707f;
const MATCH_URADD8: u32 = 0x28000077;
const MASK_URADD8: u32 = 0xfe00707f;
const MATCH_URADDW: u32 = 0x30001077;
const MASK_URADDW: u32 = 0xfe00707f;
const MATCH_URCRAS16: u32 = 0x24000077;
const MASK_URCRAS16: u32 = 0xfe00707f;
const MATCH_URCRAS32: u32 = 0x24002077;
const MASK_URCRAS32: u32 = 0xfe00707f;
const MATCH_URCRSA16: u32 = 0x26000077;
const MASK_URCRSA16: u32 = 0xfe00707f;
const MATCH_URCRSA32: u32 = 0x26002077;
const MASK_URCRSA32: u32 = 0xfe00707f;
const MATCH_URSTAS16: u32 = 0xd4002077;
const MASK_URSTAS16: u32 = 0xfe00707f;
const MATCH_URSTAS32: u32 = 0xd0002077;
const MASK_URSTAS32: u32 = 0xfe00707f;
const MATCH_URSTSA16: u32 = 0xd6002077;
const MASK_URSTSA16: u32 = 0xfe00707f;
const MATCH_URSTSA32: u32 = 0xd2002077;
const MASK_URSTSA32: u32 = 0xfe00707f;
const MATCH_URSUB16: u32 = 0x22000077;
const MASK_URSUB16: u32 = 0xfe00707f;
const MATCH_URSUB32: u32 = 0x22002077;
const MASK_URSUB32: u32 = 0xfe00707f;
const MATCH_URSUB64: u32 = 0xa2001077;
const MASK_URSUB64: u32 = 0xfe00707f;
const MATCH_URSUB8: u32 = 0x2a000077;
const MASK_URSUB8: u32 = 0xfe00707f;
const MATCH_URSUBW: u32 = 0x32001077;
const MASK_URSUBW: u32 = 0xfe00707f;
const MATCH_VAADD_VV: u32 = 0x24002057;
const MASK_VAADD_VV: u32 = 0xfc00707f;
const MATCH_VAADD_VX: u32 = 0x24006057;
const MASK_VAADD_VX: u32 = 0xfc00707f;
const MATCH_VAADDU_VV: u32 = 0x20002057;
const MASK_VAADDU_VV: u32 = 0xfc00707f;
const MATCH_VAADDU_VX: u32 = 0x20006057;
const MASK_VAADDU_VX: u32 = 0xfc00707f;
const MATCH_VADC_VIM: u32 = 0x40003057;
const MASK_VADC_VIM: u32 = 0xfe00707f;
const MATCH_VADC_VVM: u32 = 0x40000057;
const MASK_VADC_VVM: u32 = 0xfe00707f;
const MATCH_VADC_VXM: u32 = 0x40004057;
const MASK_VADC_VXM: u32 = 0xfe00707f;
const MATCH_VADD_VI: u32 = 0x3057;
const MASK_VADD_VI: u32 = 0xfc00707f;
const MATCH_VADD_VV: u32 = 0x57;
const MASK_VADD_VV: u32 = 0xfc00707f;
const MATCH_VADD_VX: u32 = 0x4057;
const MASK_VADD_VX: u32 = 0xfc00707f;
const MATCH_VAESDF_VS: u32 = 0xa600a077;
const MASK_VAESDF_VS: u32 = 0xfe0ff07f;
const MATCH_VAESDF_VV: u32 = 0xa200a077;
const MASK_VAESDF_VV: u32 = 0xfe0ff07f;
const MATCH_VAESDM_VS: u32 = 0xa6002077;
const MASK_VAESDM_VS: u32 = 0xfe0ff07f;
const MATCH_VAESDM_VV: u32 = 0xa2002077;
const MASK_VAESDM_VV: u32 = 0xfe0ff07f;
const MATCH_VAESEF_VS: u32 = 0xa601a077;
const MASK_VAESEF_VS: u32 = 0xfe0ff07f;
const MATCH_VAESEF_VV: u32 = 0xa201a077;
const MASK_VAESEF_VV: u32 = 0xfe0ff07f;
const MATCH_VAESEM_VS: u32 = 0xa6012077;
const MASK_VAESEM_VS: u32 = 0xfe0ff07f;
const MATCH_VAESEM_VV: u32 = 0xa2012077;
const MASK_VAESEM_VV: u32 = 0xfe0ff07f;
const MATCH_VAESKF1_VI: u32 = 0x8a002077;
const MASK_VAESKF1_VI: u32 = 0xfe00707f;
const MATCH_VAESKF2_VI: u32 = 0xaa002077;
const MASK_VAESKF2_VI: u32 = 0xfe00707f;
const MATCH_VAESZ_VS: u32 = 0xa603a077;
const MASK_VAESZ_VS: u32 = 0xfe0ff07f;
const MATCH_VAND_VI: u32 = 0x24003057;
const MASK_VAND_VI: u32 = 0xfc00707f;
const MATCH_VAND_VV: u32 = 0x24000057;
const MASK_VAND_VV: u32 = 0xfc00707f;
const MATCH_VAND_VX: u32 = 0x24004057;
const MASK_VAND_VX: u32 = 0xfc00707f;
const MATCH_VANDN_VV: u32 = 0x4000057;
const MASK_VANDN_VV: u32 = 0xfc00707f;
const MATCH_VANDN_VX: u32 = 0x4004057;
const MASK_VANDN_VX: u32 = 0xfc00707f;
const MATCH_VASUB_VV: u32 = 0x2c002057;
const MASK_VASUB_VV: u32 = 0xfc00707f;
const MATCH_VASUB_VX: u32 = 0x2c006057;
const MASK_VASUB_VX: u32 = 0xfc00707f;
const MATCH_VASUBU_VV: u32 = 0x28002057;
const MASK_VASUBU_VV: u32 = 0xfc00707f;
const MATCH_VASUBU_VX: u32 = 0x28006057;
const MASK_VASUBU_VX: u32 = 0xfc00707f;
const MATCH_VBREV8_V: u32 = 0x48042057;
const MASK_VBREV8_V: u32 = 0xfc0ff07f;
const MATCH_VBREV_V: u32 = 0x48052057;
const MASK_VBREV_V: u32 = 0xfc0ff07f;
const MATCH_VCLMUL_VV: u32 = 0x30002057;
const MASK_VCLMUL_VV: u32 = 0xfc00707f;
const MATCH_VCLMUL_VX: u32 = 0x30006057;
const MASK_VCLMUL_VX: u32 = 0xfc00707f;
const MATCH_VCLMULH_VV: u32 = 0x34002057;
const MASK_VCLMULH_VV: u32 = 0xfc00707f;
const MATCH_VCLMULH_VX: u32 = 0x34006057;
const MASK_VCLMULH_VX: u32 = 0xfc00707f;
const MATCH_VCLZ_V: u32 = 0x48062057;
const MASK_VCLZ_V: u32 = 0xfc0ff07f;
const MATCH_VCOMPRESS_VM: u32 = 0x5e002057;
const MASK_VCOMPRESS_VM: u32 = 0xfe00707f;
const MATCH_VCPOP_M: u32 = 0x40082057;
const MASK_VCPOP_M: u32 = 0xfc0ff07f;
const MATCH_VCPOP_V: u32 = 0x48072057;
const MASK_VCPOP_V: u32 = 0xfc0ff07f;
const MATCH_VCTZ_V: u32 = 0x4806a057;
const MASK_VCTZ_V: u32 = 0xfc0ff07f;
const MATCH_VDIV_VV: u32 = 0x84002057;
const MASK_VDIV_VV: u32 = 0xfc00707f;
const MATCH_VDIV_VX: u32 = 0x84006057;
const MASK_VDIV_VX: u32 = 0xfc00707f;
const MATCH_VDIVU_VV: u32 = 0x80002057;
const MASK_VDIVU_VV: u32 = 0xfc00707f;
const MATCH_VDIVU_VX: u32 = 0x80006057;
const MASK_VDIVU_VX: u32 = 0xfc00707f;
const MATCH_VFADD_VF: u32 = 0x5057;
const MASK_VFADD_VF: u32 = 0xfc00707f;
const MATCH_VFADD_VV: u32 = 0x1057;
const MASK_VFADD_VV: u32 = 0xfc00707f;
const MATCH_VFCLASS_V: u32 = 0x4c081057;
const MASK_VFCLASS_V: u32 = 0xfc0ff07f;
const MATCH_VFCVT_F_X_V: u32 = 0x48019057;
const MASK_VFCVT_F_X_V: u32 = 0xfc0ff07f;
const MATCH_VFCVT_F_XU_V: u32 = 0x48011057;
const MASK_VFCVT_F_XU_V: u32 = 0xfc0ff07f;
const MATCH_VFCVT_RTZ_X_F_V: u32 = 0x48039057;
const MASK_VFCVT_RTZ_X_F_V: u32 = 0xfc0ff07f;
const MATCH_VFCVT_RTZ_XU_F_V: u32 = 0x48031057;
const MASK_VFCVT_RTZ_XU_F_V: u32 = 0xfc0ff07f;
const MATCH_VFCVT_X_F_V: u32 = 0x48009057;
const MASK_VFCVT_X_F_V: u32 = 0xfc0ff07f;
const MATCH_VFCVT_XU_F_V: u32 = 0x48001057;
const MASK_VFCVT_XU_F_V: u32 = 0xfc0ff07f;
const MATCH_VFDIV_VF: u32 = 0x80005057;
const MASK_VFDIV_VF: u32 = 0xfc00707f;
const MATCH_VFDIV_VV: u32 = 0x80001057;
const MASK_VFDIV_VV: u32 = 0xfc00707f;
const MATCH_VFIRST_M: u32 = 0x4008a057;
const MASK_VFIRST_M: u32 = 0xfc0ff07f;
const MATCH_VFMACC_VF: u32 = 0xb0005057;
const MASK_VFMACC_VF: u32 = 0xfc00707f;
const MATCH_VFMACC_VV: u32 = 0xb0001057;
const MASK_VFMACC_VV: u32 = 0xfc00707f;
const MATCH_VFMADD_VF: u32 = 0xa0005057;
const MASK_VFMADD_VF: u32 = 0xfc00707f;
const MATCH_VFMADD_VV: u32 = 0xa0001057;
const MASK_VFMADD_VV: u32 = 0xfc00707f;
const MATCH_VFMAX_VF: u32 = 0x18005057;
const MASK_VFMAX_VF: u32 = 0xfc00707f;
const MATCH_VFMAX_VV: u32 = 0x18001057;
const MASK_VFMAX_VV: u32 = 0xfc00707f;
const MATCH_VFMERGE_VFM: u32 = 0x5c005057;
const MASK_VFMERGE_VFM: u32 = 0xfe00707f;
const MATCH_VFMIN_VF: u32 = 0x10005057;
const MASK_VFMIN_VF: u32 = 0xfc00707f;
const MATCH_VFMIN_VV: u32 = 0x10001057;
const MASK_VFMIN_VV: u32 = 0xfc00707f;
const MATCH_VFMSAC_VF: u32 = 0xb8005057;
const MASK_VFMSAC_VF: u32 = 0xfc00707f;
const MATCH_VFMSAC_VV: u32 = 0xb8001057;
const MASK_VFMSAC_VV: u32 = 0xfc00707f;
const MATCH_VFMSUB_VF: u32 = 0xa8005057;
const MASK_VFMSUB_VF: u32 = 0xfc00707f;
const MATCH_VFMSUB_VV: u32 = 0xa8001057;
const MASK_VFMSUB_VV: u32 = 0xfc00707f;
const MATCH_VFMUL_VF: u32 = 0x90005057;
const MASK_VFMUL_VF: u32 = 0xfc00707f;
const MATCH_VFMUL_VV: u32 = 0x90001057;
const MASK_VFMUL_VV: u32 = 0xfc00707f;
const MATCH_VFMV_F_S: u32 = 0x42001057;
const MASK_VFMV_F_S: u32 = 0xfe0ff07f;
const MATCH_VFMV_S_F: u32 = 0x42005057;
const MASK_VFMV_S_F: u32 = 0xfff0707f;
const MATCH_VFMV_V_F: u32 = 0x5e005057;
const MASK_VFMV_V_F: u32 = 0xfff0707f;
const MATCH_VFNCVT_F_F_W: u32 = 0x480a1057;
const MASK_VFNCVT_F_F_W: u32 = 0xfc0ff07f;
const MATCH_VFNCVT_F_X_W: u32 = 0x48099057;
const MASK_VFNCVT_F_X_W: u32 = 0xfc0ff07f;
const MATCH_VFNCVT_F_XU_W: u32 = 0x48091057;
const MASK_VFNCVT_F_XU_W: u32 = 0xfc0ff07f;
const MATCH_VFNCVT_ROD_F_F_W: u32 = 0x480a9057;
const MASK_VFNCVT_ROD_F_F_W: u32 = 0xfc0ff07f;
const MATCH_VFNCVT_RTZ_X_F_W: u32 = 0x480b9057;
const MASK_VFNCVT_RTZ_X_F_W: u32 = 0xfc0ff07f;
const MATCH_VFNCVT_RTZ_XU_F_W: u32 = 0x480b1057;
const MASK_VFNCVT_RTZ_XU_F_W: u32 = 0xfc0ff07f;
const MATCH_VFNCVT_X_F_W: u32 = 0x48089057;
const MASK_VFNCVT_X_F_W: u32 = 0xfc0ff07f;
const MATCH_VFNCVT_XU_F_W: u32 = 0x48081057;
const MASK_VFNCVT_XU_F_W: u32 = 0xfc0ff07f;
const MATCH_VFNCVTBF16_F_F_W: u32 = 0x480e9057;
const MASK_VFNCVTBF16_F_F_W: u32 = 0xfc0ff07f;
const MATCH_VFNMACC_VF: u32 = 0xb4005057;
const MASK_VFNMACC_VF: u32 = 0xfc00707f;
const MATCH_VFNMACC_VV: u32 = 0xb4001057;
const MASK_VFNMACC_VV: u32 = 0xfc00707f;
const MATCH_VFNMADD_VF: u32 = 0xa4005057;
const MASK_VFNMADD_VF: u32 = 0xfc00707f;
const MATCH_VFNMADD_VV: u32 = 0xa4001057;
const MASK_VFNMADD_VV: u32 = 0xfc00707f;
const MATCH_VFNMSAC_VF: u32 = 0xbc005057;
const MASK_VFNMSAC_VF: u32 = 0xfc00707f;
const MATCH_VFNMSAC_VV: u32 = 0xbc001057;
const MASK_VFNMSAC_VV: u32 = 0xfc00707f;
const MATCH_VFNMSUB_VF: u32 = 0xac005057;
const MASK_VFNMSUB_VF: u32 = 0xfc00707f;
const MATCH_VFNMSUB_VV: u32 = 0xac001057;
const MASK_VFNMSUB_VV: u32 = 0xfc00707f;
const MATCH_VFRDIV_VF: u32 = 0x84005057;
const MASK_VFRDIV_VF: u32 = 0xfc00707f;
const MATCH_VFREC7_V: u32 = 0x4c029057;
const MASK_VFREC7_V: u32 = 0xfc0ff07f;
const MATCH_VFREDMAX_VS: u32 = 0x1c001057;
const MASK_VFREDMAX_VS: u32 = 0xfc00707f;
const MATCH_VFREDMIN_VS: u32 = 0x14001057;
const MASK_VFREDMIN_VS: u32 = 0xfc00707f;
const MATCH_VFREDOSUM_VS: u32 = 0xc001057;
const MASK_VFREDOSUM_VS: u32 = 0xfc00707f;
const MATCH_VFREDSUM_VS: u32 = 0x4001057;
const MASK_VFREDSUM_VS: u32 = 0xfc00707f;
const MATCH_VFREDUSUM_VS: u32 = 0x4001057;
const MASK_VFREDUSUM_VS: u32 = 0xfc00707f;
const MATCH_VFRSQRT7_V: u32 = 0x4c021057;
const MASK_VFRSQRT7_V: u32 = 0xfc0ff07f;
const MATCH_VFRSUB_VF: u32 = 0x9c005057;
const MASK_VFRSUB_VF: u32 = 0xfc00707f;
const MATCH_VFSGNJ_VF: u32 = 0x20005057;
const MASK_VFSGNJ_VF: u32 = 0xfc00707f;
const MATCH_VFSGNJ_VV: u32 = 0x20001057;
const MASK_VFSGNJ_VV: u32 = 0xfc00707f;
const MATCH_VFSGNJN_VF: u32 = 0x24005057;
const MASK_VFSGNJN_VF: u32 = 0xfc00707f;
const MATCH_VFSGNJN_VV: u32 = 0x24001057;
const MASK_VFSGNJN_VV: u32 = 0xfc00707f;
const MATCH_VFSGNJX_VF: u32 = 0x28005057;
const MASK_VFSGNJX_VF: u32 = 0xfc00707f;
const MATCH_VFSGNJX_VV: u32 = 0x28001057;
const MASK_VFSGNJX_VV: u32 = 0xfc00707f;
const MATCH_VFSLIDE1DOWN_VF: u32 = 0x3c005057;
const MASK_VFSLIDE1DOWN_VF: u32 = 0xfc00707f;
const MATCH_VFSLIDE1UP_VF: u32 = 0x38005057;
const MASK_VFSLIDE1UP_VF: u32 = 0xfc00707f;
const MATCH_VFSQRT_V: u32 = 0x4c001057;
const MASK_VFSQRT_V: u32 = 0xfc0ff07f;
const MATCH_VFSUB_VF: u32 = 0x8005057;
const MASK_VFSUB_VF: u32 = 0xfc00707f;
const MATCH_VFSUB_VV: u32 = 0x8001057;
const MASK_VFSUB_VV: u32 = 0xfc00707f;
const MATCH_VFWADD_VF: u32 = 0xc0005057;
const MASK_VFWADD_VF: u32 = 0xfc00707f;
const MATCH_VFWADD_VV: u32 = 0xc0001057;
const MASK_VFWADD_VV: u32 = 0xfc00707f;
const MATCH_VFWADD_WF: u32 = 0xd0005057;
const MASK_VFWADD_WF: u32 = 0xfc00707f;
const MATCH_VFWADD_WV: u32 = 0xd0001057;
const MASK_VFWADD_WV: u32 = 0xfc00707f;
const MATCH_VFWCVT_F_F_V: u32 = 0x48061057;
const MASK_VFWCVT_F_F_V: u32 = 0xfc0ff07f;
const MATCH_VFWCVT_F_X_V: u32 = 0x48059057;
const MASK_VFWCVT_F_X_V: u32 = 0xfc0ff07f;
const MATCH_VFWCVT_F_XU_V: u32 = 0x48051057;
const MASK_VFWCVT_F_XU_V: u32 = 0xfc0ff07f;
const MATCH_VFWCVT_RTZ_X_F_V: u32 = 0x48079057;
const MASK_VFWCVT_RTZ_X_F_V: u32 = 0xfc0ff07f;
const MATCH_VFWCVT_RTZ_XU_F_V: u32 = 0x48071057;
const MASK_VFWCVT_RTZ_XU_F_V: u32 = 0xfc0ff07f;
const MATCH_VFWCVT_X_F_V: u32 = 0x48049057;
const MASK_VFWCVT_X_F_V: u32 = 0xfc0ff07f;
const MATCH_VFWCVT_XU_F_V: u32 = 0x48041057;
const MASK_VFWCVT_XU_F_V: u32 = 0xfc0ff07f;
const MATCH_VFWCVTBF16_F_F_V: u32 = 0x48069057;
const MASK_VFWCVTBF16_F_F_V: u32 = 0xfc0ff07f;
const MATCH_VFWMACC_VF: u32 = 0xf0005057;
const MASK_VFWMACC_VF: u32 = 0xfc00707f;
const MATCH_VFWMACC_VV: u32 = 0xf0001057;
const MASK_VFWMACC_VV: u32 = 0xfc00707f;
const MATCH_VFWMACCBF16_VF: u32 = 0xec005057;
const MASK_VFWMACCBF16_VF: u32 = 0xfc00707f;
const MATCH_VFWMACCBF16_VV: u32 = 0xec001057;
const MASK_VFWMACCBF16_VV: u32 = 0xfc00707f;
const MATCH_VFWMSAC_VF: u32 = 0xf8005057;
const MASK_VFWMSAC_VF: u32 = 0xfc00707f;
const MATCH_VFWMSAC_VV: u32 = 0xf8001057;
const MASK_VFWMSAC_VV: u32 = 0xfc00707f;
const MATCH_VFWMUL_VF: u32 = 0xe0005057;
const MASK_VFWMUL_VF: u32 = 0xfc00707f;
const MATCH_VFWMUL_VV: u32 = 0xe0001057;
const MASK_VFWMUL_VV: u32 = 0xfc00707f;
const MATCH_VFWNMACC_VF: u32 = 0xf4005057;
const MASK_VFWNMACC_VF: u32 = 0xfc00707f;
const MATCH_VFWNMACC_VV: u32 = 0xf4001057;
const MASK_VFWNMACC_VV: u32 = 0xfc00707f;
const MATCH_VFWNMSAC_VF: u32 = 0xfc005057;
const MASK_VFWNMSAC_VF: u32 = 0xfc00707f;
const MATCH_VFWNMSAC_VV: u32 = 0xfc001057;
const MASK_VFWNMSAC_VV: u32 = 0xfc00707f;
const MATCH_VFWREDOSUM_VS: u32 = 0xcc001057;
const MASK_VFWREDOSUM_VS: u32 = 0xfc00707f;
const MATCH_VFWREDSUM_VS: u32 = 0xc4001057;
const MASK_VFWREDSUM_VS: u32 = 0xfc00707f;
const MATCH_VFWREDUSUM_VS: u32 = 0xc4001057;
const MASK_VFWREDUSUM_VS: u32 = 0xfc00707f;
const MATCH_VFWSUB_VF: u32 = 0xc8005057;
const MASK_VFWSUB_VF: u32 = 0xfc00707f;
const MATCH_VFWSUB_VV: u32 = 0xc8001057;
const MASK_VFWSUB_VV: u32 = 0xfc00707f;
const MATCH_VFWSUB_WF: u32 = 0xd8005057;
const MASK_VFWSUB_WF: u32 = 0xfc00707f;
const MATCH_VFWSUB_WV: u32 = 0xd8001057;
const MASK_VFWSUB_WV: u32 = 0xfc00707f;
const MATCH_VGHSH_VV: u32 = 0xb2002077;
const MASK_VGHSH_VV: u32 = 0xfe00707f;
const MATCH_VGMUL_VV: u32 = 0xa208a077;
const MASK_VGMUL_VV: u32 = 0xfe0ff07f;
const MATCH_VID_V: u32 = 0x5008a057;
const MASK_VID_V: u32 = 0xfdfff07f;
const MATCH_VIOTA_M: u32 = 0x50082057;
const MASK_VIOTA_M: u32 = 0xfc0ff07f;
const MATCH_VL1R_V: u32 = 0x2800007;
const MASK_VL1R_V: u32 = 0xfff0707f;
const MATCH_VL1RE16_V: u32 = 0x2805007;
const MASK_VL1RE16_V: u32 = 0xfff0707f;
const MATCH_VL1RE32_V: u32 = 0x2806007;
const MASK_VL1RE32_V: u32 = 0xfff0707f;
const MATCH_VL1RE64_V: u32 = 0x2807007;
const MASK_VL1RE64_V: u32 = 0xfff0707f;
const MATCH_VL1RE8_V: u32 = 0x2800007;
const MASK_VL1RE8_V: u32 = 0xfff0707f;
const MATCH_VL2R_V: u32 = 0x22800007;
const MASK_VL2R_V: u32 = 0xfff0707f;
const MATCH_VL2RE16_V: u32 = 0x22805007;
const MASK_VL2RE16_V: u32 = 0xfff0707f;
const MATCH_VL2RE32_V: u32 = 0x22806007;
const MASK_VL2RE32_V: u32 = 0xfff0707f;
const MATCH_VL2RE64_V: u32 = 0x22807007;
const MASK_VL2RE64_V: u32 = 0xfff0707f;
const MATCH_VL2RE8_V: u32 = 0x22800007;
const MASK_VL2RE8_V: u32 = 0xfff0707f;
const MATCH_VL4R_V: u32 = 0x62800007;
const MASK_VL4R_V: u32 = 0xfff0707f;
const MATCH_VL4RE16_V: u32 = 0x62805007;
const MASK_VL4RE16_V: u32 = 0xfff0707f;
const MATCH_VL4RE32_V: u32 = 0x62806007;
const MASK_VL4RE32_V: u32 = 0xfff0707f;
const MATCH_VL4RE64_V: u32 = 0x62807007;
const MASK_VL4RE64_V: u32 = 0xfff0707f;
const MATCH_VL4RE8_V: u32 = 0x62800007;
const MASK_VL4RE8_V: u32 = 0xfff0707f;
const MATCH_VL8R_V: u32 = 0xe2800007;
const MASK_VL8R_V: u32 = 0xfff0707f;
const MATCH_VL8RE16_V: u32 = 0xe2805007;
const MASK_VL8RE16_V: u32 = 0xfff0707f;
const MATCH_VL8RE32_V: u32 = 0xe2806007;
const MASK_VL8RE32_V: u32 = 0xfff0707f;
const MATCH_VL8RE64_V: u32 = 0xe2807007;
const MASK_VL8RE64_V: u32 = 0xfff0707f;
const MATCH_VL8RE8_V: u32 = 0xe2800007;
const MASK_VL8RE8_V: u32 = 0xfff0707f;
const MATCH_VLE1024_V: u32 = 0x10007007;
const MASK_VLE1024_V: u32 = 0x1df0707f;
const MATCH_VLE1024FF_V: u32 = 0x11007007;
const MASK_VLE1024FF_V: u32 = 0x1df0707f;
const MATCH_VLE128_V: u32 = 0x10000007;
const MASK_VLE128_V: u32 = 0x1df0707f;
const MATCH_VLE128FF_V: u32 = 0x11000007;
const MASK_VLE128FF_V: u32 = 0x1df0707f;
const MATCH_VLE16_V: u32 = 0x5007;
const MASK_VLE16_V: u32 = 0x1df0707f;
const MATCH_VLE16FF_V: u32 = 0x1005007;
const MASK_VLE16FF_V: u32 = 0x1df0707f;
const MATCH_VLE1_V: u32 = 0x2b00007;
const MASK_VLE1_V: u32 = 0xfff0707f;
const MATCH_VLE256_V: u32 = 0x10005007;
const MASK_VLE256_V: u32 = 0x1df0707f;
const MATCH_VLE256FF_V: u32 = 0x11005007;
const MASK_VLE256FF_V: u32 = 0x1df0707f;
const MATCH_VLE32_V: u32 = 0x6007;
const MASK_VLE32_V: u32 = 0x1df0707f;
const MATCH_VLE32FF_V: u32 = 0x1006007;
const MASK_VLE32FF_V: u32 = 0x1df0707f;
const MATCH_VLE512_V: u32 = 0x10006007;
const MASK_VLE512_V: u32 = 0x1df0707f;
const MATCH_VLE512FF_V: u32 = 0x11006007;
const MASK_VLE512FF_V: u32 = 0x1df0707f;
const MATCH_VLE64_V: u32 = 0x7007;
const MASK_VLE64_V: u32 = 0x1df0707f;
const MATCH_VLE64FF_V: u32 = 0x1007007;
const MASK_VLE64FF_V: u32 = 0x1df0707f;
const MATCH_VLE8_V: u32 = 0x7;
const MASK_VLE8_V: u32 = 0x1df0707f;
const MATCH_VLE8FF_V: u32 = 0x1000007;
const MASK_VLE8FF_V: u32 = 0x1df0707f;
const MATCH_VLM_V: u32 = 0x2b00007;
const MASK_VLM_V: u32 = 0xfff0707f;
const MATCH_VLOXEI1024_V: u32 = 0x1c007007;
const MASK_VLOXEI1024_V: u32 = 0x1c00707f;
const MATCH_VLOXEI128_V: u32 = 0x1c000007;
const MASK_VLOXEI128_V: u32 = 0x1c00707f;
const MATCH_VLOXEI16_V: u32 = 0xc005007;
const MASK_VLOXEI16_V: u32 = 0x1c00707f;
const MATCH_VLOXEI256_V: u32 = 0x1c005007;
const MASK_VLOXEI256_V: u32 = 0x1c00707f;
const MATCH_VLOXEI32_V: u32 = 0xc006007;
const MASK_VLOXEI32_V: u32 = 0x1c00707f;
const MATCH_VLOXEI512_V: u32 = 0x1c006007;
const MASK_VLOXEI512_V: u32 = 0x1c00707f;
const MATCH_VLOXEI64_V: u32 = 0xc007007;
const MASK_VLOXEI64_V: u32 = 0x1c00707f;
const MATCH_VLOXEI8_V: u32 = 0xc000007;
const MASK_VLOXEI8_V: u32 = 0x1c00707f;
const MATCH_VLSE1024_V: u32 = 0x18007007;
const MASK_VLSE1024_V: u32 = 0x1c00707f;
const MATCH_VLSE128_V: u32 = 0x18000007;
const MASK_VLSE128_V: u32 = 0x1c00707f;
const MATCH_VLSE16_V: u32 = 0x8005007;
const MASK_VLSE16_V: u32 = 0x1c00707f;
const MATCH_VLSE256_V: u32 = 0x18005007;
const MASK_VLSE256_V: u32 = 0x1c00707f;
const MATCH_VLSE32_V: u32 = 0x8006007;
const MASK_VLSE32_V: u32 = 0x1c00707f;
const MATCH_VLSE512_V: u32 = 0x18006007;
const MASK_VLSE512_V: u32 = 0x1c00707f;
const MATCH_VLSE64_V: u32 = 0x8007007;
const MASK_VLSE64_V: u32 = 0x1c00707f;
const MATCH_VLSE8_V: u32 = 0x8000007;
const MASK_VLSE8_V: u32 = 0x1c00707f;
const MATCH_VLUXEI1024_V: u32 = 0x14007007;
const MASK_VLUXEI1024_V: u32 = 0x1c00707f;
const MATCH_VLUXEI128_V: u32 = 0x14000007;
const MASK_VLUXEI128_V: u32 = 0x1c00707f;
const MATCH_VLUXEI16_V: u32 = 0x4005007;
const MASK_VLUXEI16_V: u32 = 0x1c00707f;
const MATCH_VLUXEI256_V: u32 = 0x14005007;
const MASK_VLUXEI256_V: u32 = 0x1c00707f;
const MATCH_VLUXEI32_V: u32 = 0x4006007;
const MASK_VLUXEI32_V: u32 = 0x1c00707f;
const MATCH_VLUXEI512_V: u32 = 0x14006007;
const MASK_VLUXEI512_V: u32 = 0x1c00707f;
const MATCH_VLUXEI64_V: u32 = 0x4007007;
const MASK_VLUXEI64_V: u32 = 0x1c00707f;
const MATCH_VLUXEI8_V: u32 = 0x4000007;
const MASK_VLUXEI8_V: u32 = 0x1c00707f;
const MATCH_VMACC_VV: u32 = 0xb4002057;
const MASK_VMACC_VV: u32 = 0xfc00707f;
const MATCH_VMACC_VX: u32 = 0xb4006057;
const MASK_VMACC_VX: u32 = 0xfc00707f;
const MATCH_VMADC_VI: u32 = 0x46003057;
const MASK_VMADC_VI: u32 = 0xfe00707f;
const MATCH_VMADC_VIM: u32 = 0x44003057;
const MASK_VMADC_VIM: u32 = 0xfe00707f;
const MATCH_VMADC_VV: u32 = 0x46000057;
const MASK_VMADC_VV: u32 = 0xfe00707f;
const MATCH_VMADC_VVM: u32 = 0x44000057;
const MASK_VMADC_VVM: u32 = 0xfe00707f;
const MATCH_VMADC_VX: u32 = 0x46004057;
const MASK_VMADC_VX: u32 = 0xfe00707f;
const MATCH_VMADC_VXM: u32 = 0x44004057;
const MASK_VMADC_VXM: u32 = 0xfe00707f;
const MATCH_VMADD_VV: u32 = 0xa4002057;
const MASK_VMADD_VV: u32 = 0xfc00707f;
const MATCH_VMADD_VX: u32 = 0xa4006057;
const MASK_VMADD_VX: u32 = 0xfc00707f;
const MATCH_VMAND_MM: u32 = 0x66002057;
const MASK_VMAND_MM: u32 = 0xfe00707f;
const MATCH_VMANDN_MM: u32 = 0x62002057;
const MASK_VMANDN_MM: u32 = 0xfe00707f;
const MATCH_VMANDNOT_MM: u32 = 0x60002057;
const MASK_VMANDNOT_MM: u32 = 0xfc00707f;
const MATCH_VMAX_VV: u32 = 0x1c000057;
const MASK_VMAX_VV: u32 = 0xfc00707f;
const MATCH_VMAX_VX: u32 = 0x1c004057;
const MASK_VMAX_VX: u32 = 0xfc00707f;
const MATCH_VMAXU_VV: u32 = 0x18000057;
const MASK_VMAXU_VV: u32 = 0xfc00707f;
const MATCH_VMAXU_VX: u32 = 0x18004057;
const MASK_VMAXU_VX: u32 = 0xfc00707f;
const MATCH_VMERGE_VIM: u32 = 0x5c003057;
const MASK_VMERGE_VIM: u32 = 0xfe00707f;
const MATCH_VMERGE_VVM: u32 = 0x5c000057;
const MASK_VMERGE_VVM: u32 = 0xfe00707f;
const MATCH_VMERGE_VXM: u32 = 0x5c004057;
const MASK_VMERGE_VXM: u32 = 0xfe00707f;
const MATCH_VMFEQ_VF: u32 = 0x60005057;
const MASK_VMFEQ_VF: u32 = 0xfc00707f;
const MATCH_VMFEQ_VV: u32 = 0x60001057;
const MASK_VMFEQ_VV: u32 = 0xfc00707f;
const MATCH_VMFGE_VF: u32 = 0x7c005057;
const MASK_VMFGE_VF: u32 = 0xfc00707f;
const MATCH_VMFGT_VF: u32 = 0x74005057;
const MASK_VMFGT_VF: u32 = 0xfc00707f;
const MATCH_VMFLE_VF: u32 = 0x64005057;
const MASK_VMFLE_VF: u32 = 0xfc00707f;
const MATCH_VMFLE_VV: u32 = 0x64001057;
const MASK_VMFLE_VV: u32 = 0xfc00707f;
const MATCH_VMFLT_VF: u32 = 0x6c005057;
const MASK_VMFLT_VF: u32 = 0xfc00707f;
const MATCH_VMFLT_VV: u32 = 0x6c001057;
const MASK_VMFLT_VV: u32 = 0xfc00707f;
const MATCH_VMFNE_VF: u32 = 0x70005057;
const MASK_VMFNE_VF: u32 = 0xfc00707f;
const MATCH_VMFNE_VV: u32 = 0x70001057;
const MASK_VMFNE_VV: u32 = 0xfc00707f;
const MATCH_VMIN_VV: u32 = 0x14000057;
const MASK_VMIN_VV: u32 = 0xfc00707f;
const MATCH_VMIN_VX: u32 = 0x14004057;
const MASK_VMIN_VX: u32 = 0xfc00707f;
const MATCH_VMINU_VV: u32 = 0x10000057;
const MASK_VMINU_VV: u32 = 0xfc00707f;
const MATCH_VMINU_VX: u32 = 0x10004057;
const MASK_VMINU_VX: u32 = 0xfc00707f;
const MATCH_VMNAND_MM: u32 = 0x76002057;
const MASK_VMNAND_MM: u32 = 0xfe00707f;
const MATCH_VMNOR_MM: u32 = 0x7a002057;
const MASK_VMNOR_MM: u32 = 0xfe00707f;
const MATCH_VMOR_MM: u32 = 0x6a002057;
const MASK_VMOR_MM: u32 = 0xfe00707f;
const MATCH_VMORN_MM: u32 = 0x72002057;
const MASK_VMORN_MM: u32 = 0xfe00707f;
const MATCH_VMORNOT_MM: u32 = 0x70002057;
const MASK_VMORNOT_MM: u32 = 0xfc00707f;
const MATCH_VMSBC_VV: u32 = 0x4e000057;
const MASK_VMSBC_VV: u32 = 0xfe00707f;
const MATCH_VMSBC_VVM: u32 = 0x4c000057;
const MASK_VMSBC_VVM: u32 = 0xfe00707f;
const MATCH_VMSBC_VX: u32 = 0x4e004057;
const MASK_VMSBC_VX: u32 = 0xfe00707f;
const MATCH_VMSBC_VXM: u32 = 0x4c004057;
const MASK_VMSBC_VXM: u32 = 0xfe00707f;
const MATCH_VMSBF_M: u32 = 0x5000a057;
const MASK_VMSBF_M: u32 = 0xfc0ff07f;
const MATCH_VMSEQ_VI: u32 = 0x60003057;
const MASK_VMSEQ_VI: u32 = 0xfc00707f;
const MATCH_VMSEQ_VV: u32 = 0x60000057;
const MASK_VMSEQ_VV: u32 = 0xfc00707f;
const MATCH_VMSEQ_VX: u32 = 0x60004057;
const MASK_VMSEQ_VX: u32 = 0xfc00707f;
const MATCH_VMSGT_VI: u32 = 0x7c003057;
const MASK_VMSGT_VI: u32 = 0xfc00707f;
const MATCH_VMSGT_VX: u32 = 0x7c004057;
const MASK_VMSGT_VX: u32 = 0xfc00707f;
const MATCH_VMSGTU_VI: u32 = 0x78003057;
const MASK_VMSGTU_VI: u32 = 0xfc00707f;
const MATCH_VMSGTU_VX: u32 = 0x78004057;
const MASK_VMSGTU_VX: u32 = 0xfc00707f;
const MATCH_VMSIF_M: u32 = 0x5001a057;
const MASK_VMSIF_M: u32 = 0xfc0ff07f;
const MATCH_VMSLE_VI: u32 = 0x74003057;
const MASK_VMSLE_VI: u32 = 0xfc00707f;
const MATCH_VMSLE_VV: u32 = 0x74000057;
const MASK_VMSLE_VV: u32 = 0xfc00707f;
const MATCH_VMSLE_VX: u32 = 0x74004057;
const MASK_VMSLE_VX: u32 = 0xfc00707f;
const MATCH_VMSLEU_VI: u32 = 0x70003057;
const MASK_VMSLEU_VI: u32 = 0xfc00707f;
const MATCH_VMSLEU_VV: u32 = 0x70000057;
const MASK_VMSLEU_VV: u32 = 0xfc00707f;
const MATCH_VMSLEU_VX: u32 = 0x70004057;
const MASK_VMSLEU_VX: u32 = 0xfc00707f;
const MATCH_VMSLT_VV: u32 = 0x6c000057;
const MASK_VMSLT_VV: u32 = 0xfc00707f;
const MATCH_VMSLT_VX: u32 = 0x6c004057;
const MASK_VMSLT_VX: u32 = 0xfc00707f;
const MATCH_VMSLTU_VV: u32 = 0x68000057;
const MASK_VMSLTU_VV: u32 = 0xfc00707f;
const MATCH_VMSLTU_VX: u32 = 0x68004057;
const MASK_VMSLTU_VX: u32 = 0xfc00707f;
const MATCH_VMSNE_VI: u32 = 0x64003057;
const MASK_VMSNE_VI: u32 = 0xfc00707f;
const MATCH_VMSNE_VV: u32 = 0x64000057;
const MASK_VMSNE_VV: u32 = 0xfc00707f;
const MATCH_VMSNE_VX: u32 = 0x64004057;
const MASK_VMSNE_VX: u32 = 0xfc00707f;
const MATCH_VMSOF_M: u32 = 0x50012057;
const MASK_VMSOF_M: u32 = 0xfc0ff07f;
const MATCH_VMUL_VV: u32 = 0x94002057;
const MASK_VMUL_VV: u32 = 0xfc00707f;
const MATCH_VMUL_VX: u32 = 0x94006057;
const MASK_VMUL_VX: u32 = 0xfc00707f;
const MATCH_VMULH_VV: u32 = 0x9c002057;
const MASK_VMULH_VV: u32 = 0xfc00707f;
const MATCH_VMULH_VX: u32 = 0x9c006057;
const MASK_VMULH_VX: u32 = 0xfc00707f;
const MATCH_VMULHSU_VV: u32 = 0x98002057;
const MASK_VMULHSU_VV: u32 = 0xfc00707f;
const MATCH_VMULHSU_VX: u32 = 0x98006057;
const MASK_VMULHSU_VX: u32 = 0xfc00707f;
const MATCH_VMULHU_VV: u32 = 0x90002057;
const MASK_VMULHU_VV: u32 = 0xfc00707f;
const MATCH_VMULHU_VX: u32 = 0x90006057;
const MASK_VMULHU_VX: u32 = 0xfc00707f;
const MATCH_VMV1R_V: u32 = 0x9e003057;
const MASK_VMV1R_V: u32 = 0xfe0ff07f;
const MATCH_VMV2R_V: u32 = 0x9e00b057;
const MASK_VMV2R_V: u32 = 0xfe0ff07f;
const MATCH_VMV4R_V: u32 = 0x9e01b057;
const MASK_VMV4R_V: u32 = 0xfe0ff07f;
const MATCH_VMV8R_V: u32 = 0x9e03b057;
const MASK_VMV8R_V: u32 = 0xfe0ff07f;
const MATCH_VMV_S_X: u32 = 0x42006057;
const MASK_VMV_S_X: u32 = 0xfff0707f;
const MATCH_VMV_V_I: u32 = 0x5e003057;
const MASK_VMV_V_I: u32 = 0xfff0707f;
const MATCH_VMV_V_V: u32 = 0x5e000057;
const MASK_VMV_V_V: u32 = 0xfff0707f;
const MATCH_VMV_V_X: u32 = 0x5e004057;
const MASK_VMV_V_X: u32 = 0xfff0707f;
const MATCH_VMV_X_S: u32 = 0x42002057;
const MASK_VMV_X_S: u32 = 0xfe0ff07f;
const MATCH_VMXNOR_MM: u32 = 0x7e002057;
const MASK_VMXNOR_MM: u32 = 0xfe00707f;
const MATCH_VMXOR_MM: u32 = 0x6e002057;
const MASK_VMXOR_MM: u32 = 0xfe00707f;
const MATCH_VNCLIP_WI: u32 = 0xbc003057;
const MASK_VNCLIP_WI: u32 = 0xfc00707f;
const MATCH_VNCLIP_WV: u32 = 0xbc000057;
const MASK_VNCLIP_WV: u32 = 0xfc00707f;
const MATCH_VNCLIP_WX: u32 = 0xbc004057;
const MASK_VNCLIP_WX: u32 = 0xfc00707f;
const MATCH_VNCLIPU_WI: u32 = 0xb8003057;
const MASK_VNCLIPU_WI: u32 = 0xfc00707f;
const MATCH_VNCLIPU_WV: u32 = 0xb8000057;
const MASK_VNCLIPU_WV: u32 = 0xfc00707f;
const MATCH_VNCLIPU_WX: u32 = 0xb8004057;
const MASK_VNCLIPU_WX: u32 = 0xfc00707f;
const MATCH_VNMSAC_VV: u32 = 0xbc002057;
const MASK_VNMSAC_VV: u32 = 0xfc00707f;
const MATCH_VNMSAC_VX: u32 = 0xbc006057;
const MASK_VNMSAC_VX: u32 = 0xfc00707f;
const MATCH_VNMSUB_VV: u32 = 0xac002057;
const MASK_VNMSUB_VV: u32 = 0xfc00707f;
const MATCH_VNMSUB_VX: u32 = 0xac006057;
const MASK_VNMSUB_VX: u32 = 0xfc00707f;
const MATCH_VNSRA_WI: u32 = 0xb4003057;
const MASK_VNSRA_WI: u32 = 0xfc00707f;
const MATCH_VNSRA_WV: u32 = 0xb4000057;
const MASK_VNSRA_WV: u32 = 0xfc00707f;
const MATCH_VNSRA_WX: u32 = 0xb4004057;
const MASK_VNSRA_WX: u32 = 0xfc00707f;
const MATCH_VNSRL_WI: u32 = 0xb0003057;
const MASK_VNSRL_WI: u32 = 0xfc00707f;
const MATCH_VNSRL_WV: u32 = 0xb0000057;
const MASK_VNSRL_WV: u32 = 0xfc00707f;
const MATCH_VNSRL_WX: u32 = 0xb0004057;
const MASK_VNSRL_WX: u32 = 0xfc00707f;
const MATCH_VOR_VI: u32 = 0x28003057;
const MASK_VOR_VI: u32 = 0xfc00707f;
const MATCH_VOR_VV: u32 = 0x28000057;
const MASK_VOR_VV: u32 = 0xfc00707f;
const MATCH_VOR_VX: u32 = 0x28004057;
const MASK_VOR_VX: u32 = 0xfc00707f;
const MATCH_VPOPC_M: u32 = 0x40082057;
const MASK_VPOPC_M: u32 = 0xfc0ff07f;
const MATCH_VREDAND_VS: u32 = 0x4002057;
const MASK_VREDAND_VS: u32 = 0xfc00707f;
const MATCH_VREDMAX_VS: u32 = 0x1c002057;
const MASK_VREDMAX_VS: u32 = 0xfc00707f;
const MATCH_VREDMAXU_VS: u32 = 0x18002057;
const MASK_VREDMAXU_VS: u32 = 0xfc00707f;
const MATCH_VREDMIN_VS: u32 = 0x14002057;
const MASK_VREDMIN_VS: u32 = 0xfc00707f;
const MATCH_VREDMINU_VS: u32 = 0x10002057;
const MASK_VREDMINU_VS: u32 = 0xfc00707f;
const MATCH_VREDOR_VS: u32 = 0x8002057;
const MASK_VREDOR_VS: u32 = 0xfc00707f;
const MATCH_VREDSUM_VS: u32 = 0x2057;
const MASK_VREDSUM_VS: u32 = 0xfc00707f;
const MATCH_VREDXOR_VS: u32 = 0xc002057;
const MASK_VREDXOR_VS: u32 = 0xfc00707f;
const MATCH_VREM_VV: u32 = 0x8c002057;
const MASK_VREM_VV: u32 = 0xfc00707f;
const MATCH_VREM_VX: u32 = 0x8c006057;
const MASK_VREM_VX: u32 = 0xfc00707f;
const MATCH_VREMU_VV: u32 = 0x88002057;
const MASK_VREMU_VV: u32 = 0xfc00707f;
const MATCH_VREMU_VX: u32 = 0x88006057;
const MASK_VREMU_VX: u32 = 0xfc00707f;
const MATCH_VREV8_V: u32 = 0x4804a057;
const MASK_VREV8_V: u32 = 0xfc0ff07f;
const MATCH_VRGATHER_VI: u32 = 0x30003057;
const MASK_VRGATHER_VI: u32 = 0xfc00707f;
const MATCH_VRGATHER_VV: u32 = 0x30000057;
const MASK_VRGATHER_VV: u32 = 0xfc00707f;
const MATCH_VRGATHER_VX: u32 = 0x30004057;
const MASK_VRGATHER_VX: u32 = 0xfc00707f;
const MATCH_VRGATHEREI16_VV: u32 = 0x38000057;
const MASK_VRGATHEREI16_VV: u32 = 0xfc00707f;
const MATCH_VROL_VV: u32 = 0x54000057;
const MASK_VROL_VV: u32 = 0xfc00707f;
const MATCH_VROL_VX: u32 = 0x54004057;
const MASK_VROL_VX: u32 = 0xfc00707f;
const MATCH_VROR_VI: u32 = 0x50003057;
const MASK_VROR_VI: u32 = 0xf800707f;
const MATCH_VROR_VV: u32 = 0x50000057;
const MASK_VROR_VV: u32 = 0xfc00707f;
const MATCH_VROR_VX: u32 = 0x50004057;
const MASK_VROR_VX: u32 = 0xfc00707f;
const MATCH_VRSUB_VI: u32 = 0xc003057;
const MASK_VRSUB_VI: u32 = 0xfc00707f;
const MATCH_VRSUB_VX: u32 = 0xc004057;
const MASK_VRSUB_VX: u32 = 0xfc00707f;
const MATCH_VS1R_V: u32 = 0x2800027;
const MASK_VS1R_V: u32 = 0xfff0707f;
const MATCH_VS2R_V: u32 = 0x22800027;
const MASK_VS2R_V: u32 = 0xfff0707f;
const MATCH_VS4R_V: u32 = 0x62800027;
const MASK_VS4R_V: u32 = 0xfff0707f;
const MATCH_VS8R_V: u32 = 0xe2800027;
const MASK_VS8R_V: u32 = 0xfff0707f;
const MATCH_VSADD_VI: u32 = 0x84003057;
const MASK_VSADD_VI: u32 = 0xfc00707f;
const MATCH_VSADD_VV: u32 = 0x84000057;
const MASK_VSADD_VV: u32 = 0xfc00707f;
const MATCH_VSADD_VX: u32 = 0x84004057;
const MASK_VSADD_VX: u32 = 0xfc00707f;
const MATCH_VSADDU_VI: u32 = 0x80003057;
const MASK_VSADDU_VI: u32 = 0xfc00707f;
const MATCH_VSADDU_VV: u32 = 0x80000057;
const MASK_VSADDU_VV: u32 = 0xfc00707f;
const MATCH_VSADDU_VX: u32 = 0x80004057;
const MASK_VSADDU_VX: u32 = 0xfc00707f;
const MATCH_VSBC_VVM: u32 = 0x48000057;
const MASK_VSBC_VVM: u32 = 0xfe00707f;
const MATCH_VSBC_VXM: u32 = 0x48004057;
const MASK_VSBC_VXM: u32 = 0xfe00707f;
const MATCH_VSE1024_V: u32 = 0x10007027;
const MASK_VSE1024_V: u32 = 0x1df0707f;
const MATCH_VSE128_V: u32 = 0x10000027;
const MASK_VSE128_V: u32 = 0x1df0707f;
const MATCH_VSE16_V: u32 = 0x5027;
const MASK_VSE16_V: u32 = 0x1df0707f;
const MATCH_VSE1_V: u32 = 0x2b00027;
const MASK_VSE1_V: u32 = 0xfff0707f;
const MATCH_VSE256_V: u32 = 0x10005027;
const MASK_VSE256_V: u32 = 0x1df0707f;
const MATCH_VSE32_V: u32 = 0x6027;
const MASK_VSE32_V: u32 = 0x1df0707f;
const MATCH_VSE512_V: u32 = 0x10006027;
const MASK_VSE512_V: u32 = 0x1df0707f;
const MATCH_VSE64_V: u32 = 0x7027;
const MASK_VSE64_V: u32 = 0x1df0707f;
const MATCH_VSE8_V: u32 = 0x27;
const MASK_VSE8_V: u32 = 0x1df0707f;
const MATCH_VSETIVLI: u32 = 0xc0007057;
const MASK_VSETIVLI: u32 = 0xc000707f;
const MATCH_VSETVL: u32 = 0x80007057;
const MASK_VSETVL: u32 = 0xfe00707f;
const MATCH_VSETVLI: u32 = 0x7057;
const MASK_VSETVLI: u32 = 0x8000707f;
const MATCH_VSEXT_VF2: u32 = 0x4803a057;
const MASK_VSEXT_VF2: u32 = 0xfc0ff07f;
const MATCH_VSEXT_VF4: u32 = 0x4802a057;
const MASK_VSEXT_VF4: u32 = 0xfc0ff07f;
const MATCH_VSEXT_VF8: u32 = 0x4801a057;
const MASK_VSEXT_VF8: u32 = 0xfc0ff07f;
const MATCH_VSHA2CH_VV: u32 = 0xba002077;
const MASK_VSHA2CH_VV: u32 = 0xfe00707f;
const MATCH_VSHA2CL_VV: u32 = 0xbe002077;
const MASK_VSHA2CL_VV: u32 = 0xfe00707f;
const MATCH_VSHA2MS_VV: u32 = 0xb6002077;
const MASK_VSHA2MS_VV: u32 = 0xfe00707f;
const MATCH_VSLIDE1DOWN_VX: u32 = 0x3c006057;
const MASK_VSLIDE1DOWN_VX: u32 = 0xfc00707f;
const MATCH_VSLIDE1UP_VX: u32 = 0x38006057;
const MASK_VSLIDE1UP_VX: u32 = 0xfc00707f;
const MATCH_VSLIDEDOWN_VI: u32 = 0x3c003057;
const MASK_VSLIDEDOWN_VI: u32 = 0xfc00707f;
const MATCH_VSLIDEDOWN_VX: u32 = 0x3c004057;
const MASK_VSLIDEDOWN_VX: u32 = 0xfc00707f;
const MATCH_VSLIDEUP_VI: u32 = 0x38003057;
const MASK_VSLIDEUP_VI: u32 = 0xfc00707f;
const MATCH_VSLIDEUP_VX: u32 = 0x38004057;
const MASK_VSLIDEUP_VX: u32 = 0xfc00707f;
const MATCH_VSLL_VI: u32 = 0x94003057;
const MASK_VSLL_VI: u32 = 0xfc00707f;
const MATCH_VSLL_VV: u32 = 0x94000057;
const MASK_VSLL_VV: u32 = 0xfc00707f;
const MATCH_VSLL_VX: u32 = 0x94004057;
const MASK_VSLL_VX: u32 = 0xfc00707f;
const MATCH_VSM3C_VI: u32 = 0xae002077;
const MASK_VSM3C_VI: u32 = 0xfe00707f;
const MATCH_VSM3ME_VV: u32 = 0x82002077;
const MASK_VSM3ME_VV: u32 = 0xfe00707f;
const MATCH_VSM4K_VI: u32 = 0x86002077;
const MASK_VSM4K_VI: u32 = 0xfe00707f;
const MATCH_VSM4R_VS: u32 = 0xa6082077;
const MASK_VSM4R_VS: u32 = 0xfe0ff07f;
const MATCH_VSM4R_VV: u32 = 0xa2082077;
const MASK_VSM4R_VV: u32 = 0xfe0ff07f;
const MATCH_VSM_V: u32 = 0x2b00027;
const MASK_VSM_V: u32 = 0xfff0707f;
const MATCH_VSMUL_VV: u32 = 0x9c000057;
const MASK_VSMUL_VV: u32 = 0xfc00707f;
const MATCH_VSMUL_VX: u32 = 0x9c004057;
const MASK_VSMUL_VX: u32 = 0xfc00707f;
const MATCH_VSOXEI1024_V: u32 = 0x1c007027;
const MASK_VSOXEI1024_V: u32 = 0x1c00707f;
const MATCH_VSOXEI128_V: u32 = 0x1c000027;
const MASK_VSOXEI128_V: u32 = 0x1c00707f;
const MATCH_VSOXEI16_V: u32 = 0xc005027;
const MASK_VSOXEI16_V: u32 = 0x1c00707f;
const MATCH_VSOXEI256_V: u32 = 0x1c005027;
const MASK_VSOXEI256_V: u32 = 0x1c00707f;
const MATCH_VSOXEI32_V: u32 = 0xc006027;
const MASK_VSOXEI32_V: u32 = 0x1c00707f;
const MATCH_VSOXEI512_V: u32 = 0x1c006027;
const MASK_VSOXEI512_V: u32 = 0x1c00707f;
const MATCH_VSOXEI64_V: u32 = 0xc007027;
const MASK_VSOXEI64_V: u32 = 0x1c00707f;
const MATCH_VSOXEI8_V: u32 = 0xc000027;
const MASK_VSOXEI8_V: u32 = 0x1c00707f;
const MATCH_VSRA_VI: u32 = 0xa4003057;
const MASK_VSRA_VI: u32 = 0xfc00707f;
const MATCH_VSRA_VV: u32 = 0xa4000057;
const MASK_VSRA_VV: u32 = 0xfc00707f;
const MATCH_VSRA_VX: u32 = 0xa4004057;
const MASK_VSRA_VX: u32 = 0xfc00707f;
const MATCH_VSRL_VI: u32 = 0xa0003057;
const MASK_VSRL_VI: u32 = 0xfc00707f;
const MATCH_VSRL_VV: u32 = 0xa0000057;
const MASK_VSRL_VV: u32 = 0xfc00707f;
const MATCH_VSRL_VX: u32 = 0xa0004057;
const MASK_VSRL_VX: u32 = 0xfc00707f;
const MATCH_VSSE1024_V: u32 = 0x18007027;
const MASK_VSSE1024_V: u32 = 0x1c00707f;
const MATCH_VSSE128_V: u32 = 0x18000027;
const MASK_VSSE128_V: u32 = 0x1c00707f;
const MATCH_VSSE16_V: u32 = 0x8005027;
const MASK_VSSE16_V: u32 = 0x1c00707f;
const MATCH_VSSE256_V: u32 = 0x18005027;
const MASK_VSSE256_V: u32 = 0x1c00707f;
const MATCH_VSSE32_V: u32 = 0x8006027;
const MASK_VSSE32_V: u32 = 0x1c00707f;
const MATCH_VSSE512_V: u32 = 0x18006027;
const MASK_VSSE512_V: u32 = 0x1c00707f;
const MATCH_VSSE64_V: u32 = 0x8007027;
const MASK_VSSE64_V: u32 = 0x1c00707f;
const MATCH_VSSE8_V: u32 = 0x8000027;
const MASK_VSSE8_V: u32 = 0x1c00707f;
const MATCH_VSSRA_VI: u32 = 0xac003057;
const MASK_VSSRA_VI: u32 = 0xfc00707f;
const MATCH_VSSRA_VV: u32 = 0xac000057;
const MASK_VSSRA_VV: u32 = 0xfc00707f;
const MATCH_VSSRA_VX: u32 = 0xac004057;
const MASK_VSSRA_VX: u32 = 0xfc00707f;
const MATCH_VSSRL_VI: u32 = 0xa8003057;
const MASK_VSSRL_VI: u32 = 0xfc00707f;
const MATCH_VSSRL_VV: u32 = 0xa8000057;
const MASK_VSSRL_VV: u32 = 0xfc00707f;
const MATCH_VSSRL_VX: u32 = 0xa8004057;
const MASK_VSSRL_VX: u32 = 0xfc00707f;
const MATCH_VSSUB_VV: u32 = 0x8c000057;
const MASK_VSSUB_VV: u32 = 0xfc00707f;
const MATCH_VSSUB_VX: u32 = 0x8c004057;
const MASK_VSSUB_VX: u32 = 0xfc00707f;
const MATCH_VSSUBU_VV: u32 = 0x88000057;
const MASK_VSSUBU_VV: u32 = 0xfc00707f;
const MATCH_VSSUBU_VX: u32 = 0x88004057;
const MASK_VSSUBU_VX: u32 = 0xfc00707f;
const MATCH_VSUB_VV: u32 = 0x8000057;
const MASK_VSUB_VV: u32 = 0xfc00707f;
const MATCH_VSUB_VX: u32 = 0x8004057;
const MASK_VSUB_VX: u32 = 0xfc00707f;
const MATCH_VSUXEI1024_V: u32 = 0x14007027;
const MASK_VSUXEI1024_V: u32 = 0x1c00707f;
const MATCH_VSUXEI128_V: u32 = 0x14000027;
const MASK_VSUXEI128_V: u32 = 0x1c00707f;
const MATCH_VSUXEI16_V: u32 = 0x4005027;
const MASK_VSUXEI16_V: u32 = 0x1c00707f;
const MATCH_VSUXEI256_V: u32 = 0x14005027;
const MASK_VSUXEI256_V: u32 = 0x1c00707f;
const MATCH_VSUXEI32_V: u32 = 0x4006027;
const MASK_VSUXEI32_V: u32 = 0x1c00707f;
const MATCH_VSUXEI512_V: u32 = 0x14006027;
const MASK_VSUXEI512_V: u32 = 0x1c00707f;
const MATCH_VSUXEI64_V: u32 = 0x4007027;
const MASK_VSUXEI64_V: u32 = 0x1c00707f;
const MATCH_VSUXEI8_V: u32 = 0x4000027;
const MASK_VSUXEI8_V: u32 = 0x1c00707f;
const MATCH_VWADD_VV: u32 = 0xc4002057;
const MASK_VWADD_VV: u32 = 0xfc00707f;
const MATCH_VWADD_VX: u32 = 0xc4006057;
const MASK_VWADD_VX: u32 = 0xfc00707f;
const MATCH_VWADD_WV: u32 = 0xd4002057;
const MASK_VWADD_WV: u32 = 0xfc00707f;
const MATCH_VWADD_WX: u32 = 0xd4006057;
const MASK_VWADD_WX: u32 = 0xfc00707f;
const MATCH_VWADDU_VV: u32 = 0xc0002057;
const MASK_VWADDU_VV: u32 = 0xfc00707f;
const MATCH_VWADDU_VX: u32 = 0xc0006057;
const MASK_VWADDU_VX: u32 = 0xfc00707f;
const MATCH_VWADDU_WV: u32 = 0xd0002057;
const MASK_VWADDU_WV: u32 = 0xfc00707f;
const MATCH_VWADDU_WX: u32 = 0xd0006057;
const MASK_VWADDU_WX: u32 = 0xfc00707f;
const MATCH_VWMACC_VV: u32 = 0xf4002057;
const MASK_VWMACC_VV: u32 = 0xfc00707f;
const MATCH_VWMACC_VX: u32 = 0xf4006057;
const MASK_VWMACC_VX: u32 = 0xfc00707f;
const MATCH_VWMACCSU_VV: u32 = 0xfc002057;
const MASK_VWMACCSU_VV: u32 = 0xfc00707f;
const MATCH_VWMACCSU_VX: u32 = 0xfc006057;
const MASK_VWMACCSU_VX: u32 = 0xfc00707f;
const MATCH_VWMACCU_VV: u32 = 0xf0002057;
const MASK_VWMACCU_VV: u32 = 0xfc00707f;
const MATCH_VWMACCU_VX: u32 = 0xf0006057;
const MASK_VWMACCU_VX: u32 = 0xfc00707f;
const MATCH_VWMACCUS_VX: u32 = 0xf8006057;
const MASK_VWMACCUS_VX: u32 = 0xfc00707f;
const MATCH_VWMUL_VV: u32 = 0xec002057;
const MASK_VWMUL_VV: u32 = 0xfc00707f;
const MATCH_VWMUL_VX: u32 = 0xec006057;
const MASK_VWMUL_VX: u32 = 0xfc00707f;
const MATCH_VWMULSU_VV: u32 = 0xe8002057;
const MASK_VWMULSU_VV: u32 = 0xfc00707f;
const MATCH_VWMULSU_VX: u32 = 0xe8006057;
const MASK_VWMULSU_VX: u32 = 0xfc00707f;
const MATCH_VWMULU_VV: u32 = 0xe0002057;
const MASK_VWMULU_VV: u32 = 0xfc00707f;
const MATCH_VWMULU_VX: u32 = 0xe0006057;
const MASK_VWMULU_VX: u32 = 0xfc00707f;
const MATCH_VWREDSUM_VS: u32 = 0xc4000057;
const MASK_VWREDSUM_VS: u32 = 0xfc00707f;
const MATCH_VWREDSUMU_VS: u32 = 0xc0000057;
const MASK_VWREDSUMU_VS: u32 = 0xfc00707f;
const MATCH_VWSLL_VI: u32 = 0xd4003057;
const MASK_VWSLL_VI: u32 = 0xfc00707f;
const MATCH_VWSLL_VV: u32 = 0xd4000057;
const MASK_VWSLL_VV: u32 = 0xfc00707f;
const MATCH_VWSLL_VX: u32 = 0xd4004057;
const MASK_VWSLL_VX: u32 = 0xfc00707f;
const MATCH_VWSUB_VV: u32 = 0xcc002057;
const MASK_VWSUB_VV: u32 = 0xfc00707f;
const MATCH_VWSUB_VX: u32 = 0xcc006057;
const MASK_VWSUB_VX: u32 = 0xfc00707f;
const MATCH_VWSUB_WV: u32 = 0xdc002057;
const MASK_VWSUB_WV: u32 = 0xfc00707f;
const MATCH_VWSUB_WX: u32 = 0xdc006057;
const MASK_VWSUB_WX: u32 = 0xfc00707f;
const MATCH_VWSUBU_VV: u32 = 0xc8002057;
const MASK_VWSUBU_VV: u32 = 0xfc00707f;
const MATCH_VWSUBU_VX: u32 = 0xc8006057;
const MASK_VWSUBU_VX: u32 = 0xfc00707f;
const MATCH_VWSUBU_WV: u32 = 0xd8002057;
const MASK_VWSUBU_WV: u32 = 0xfc00707f;
const MATCH_VWSUBU_WX: u32 = 0xd8006057;
const MASK_VWSUBU_WX: u32 = 0xfc00707f;
const MATCH_VXOR_VI: u32 = 0x2c003057;
const MASK_VXOR_VI: u32 = 0xfc00707f;
const MATCH_VXOR_VV: u32 = 0x2c000057;
const MASK_VXOR_VV: u32 = 0xfc00707f;
const MATCH_VXOR_VX: u32 = 0x2c004057;
const MASK_VXOR_VX: u32 = 0xfc00707f;
const MATCH_VZEXT_VF2: u32 = 0x48032057;
const MASK_VZEXT_VF2: u32 = 0xfc0ff07f;
const MATCH_VZEXT_VF4: u32 = 0x48022057;
const MASK_VZEXT_VF4: u32 = 0xfc0ff07f;
const MATCH_VZEXT_VF8: u32 = 0x48012057;
const MASK_VZEXT_VF8: u32 = 0xfc0ff07f;
const MATCH_WFI: u32 = 0x10500073;
const MASK_WFI: u32 = 0xffffffff;
const MATCH_WRS_NTO: u32 = 0xd00073;
const MASK_WRS_NTO: u32 = 0xffffffff;
const MATCH_WRS_STO: u32 = 0x1d00073;
const MASK_WRS_STO: u32 = 0xffffffff;
const MATCH_XNOR: u32 = 0x40004033;
const MASK_XNOR: u32 = 0xfe00707f;
const MATCH_XOR: u32 = 0x4033;
const MASK_XOR: u32 = 0xfe00707f;
const MATCH_XORI: u32 = 0x4013;
const MASK_XORI: u32 = 0x707f;
const MATCH_XPERM16: u32 = 0x28006033;
const MASK_XPERM16: u32 = 0xfe00707f;
const MATCH_XPERM32: u32 = 0x28000033;
const MASK_XPERM32: u32 = 0xfe00707f;
const MATCH_XPERM4: u32 = 0x28002033;
const MASK_XPERM4: u32 = 0xfe00707f;
const MATCH_XPERM8: u32 = 0x28004033;
const MASK_XPERM8: u32 = 0xfe00707f;
const MATCH_ZEXT_H: u32 = 0x800403b;
const MASK_ZEXT_H: u32 = 0xfff0707f;
const MATCH_ZEXT_H_RV32: u32 = 0x8004033;
const MASK_ZEXT_H_RV32: u32 = 0xfff0707f;
const MATCH_ZIP: u32 = 0x8f01013;
const MASK_ZIP: u32 = 0xfff0707f;
const MATCH_ZUNPKD810: u32 = 0xacc00077;
const MASK_ZUNPKD810: u32 = 0xfff0707f;
const MATCH_ZUNPKD820: u32 = 0xacd00077;
const MASK_ZUNPKD820: u32 = 0xfff0707f;
const MATCH_ZUNPKD830: u32 = 0xace00077;
const MASK_ZUNPKD830: u32 = 0xfff0707f;
const MATCH_ZUNPKD831: u32 = 0xacf00077;
const MASK_ZUNPKD831: u32 = 0xfff0707f;
const MATCH_ZUNPKD832: u32 = 0xad700077;
const MASK_ZUNPKD832: u32 = 0xfff0707f;
const CSR_FFLAGS: u16 = 0x1;
const CSR_FRM: u16 = 0x2;
const CSR_FCSR: u16 = 0x3;
const CSR_VSTART: u16 = 0x8;
const CSR_VXSAT: u16 = 0x9;
const CSR_VXRM: u16 = 0xa;
const CSR_VCSR: u16 = 0xf;
const CSR_SSP: u16 = 0x11;
const CSR_SEED: u16 = 0x15;
const CSR_JVT: u16 = 0x17;
const CSR_CYCLE: u16 = 0xc00;
const CSR_TIME: u16 = 0xc01;
const CSR_INSTRET: u16 = 0xc02;
const CSR_HPMCOUNTER3: u16 = 0xc03;
const CSR_HPMCOUNTER4: u16 = 0xc04;
const CSR_HPMCOUNTER5: u16 = 0xc05;
const CSR_HPMCOUNTER6: u16 = 0xc06;
const CSR_HPMCOUNTER7: u16 = 0xc07;
const CSR_HPMCOUNTER8: u16 = 0xc08;
const CSR_HPMCOUNTER9: u16 = 0xc09;
const CSR_HPMCOUNTER10: u16 = 0xc0a;
const CSR_HPMCOUNTER11: u16 = 0xc0b;
const CSR_HPMCOUNTER12: u16 = 0xc0c;
const CSR_HPMCOUNTER13: u16 = 0xc0d;
const CSR_HPMCOUNTER14: u16 = 0xc0e;
const CSR_HPMCOUNTER15: u16 = 0xc0f;
const CSR_HPMCOUNTER16: u16 = 0xc10;
const CSR_HPMCOUNTER17: u16 = 0xc11;
const CSR_HPMCOUNTER18: u16 = 0xc12;
const CSR_HPMCOUNTER19: u16 = 0xc13;
const CSR_HPMCOUNTER20: u16 = 0xc14;
const CSR_HPMCOUNTER21: u16 = 0xc15;
const CSR_HPMCOUNTER22: u16 = 0xc16;
const CSR_HPMCOUNTER23: u16 = 0xc17;
const CSR_HPMCOUNTER24: u16 = 0xc18;
const CSR_HPMCOUNTER25: u16 = 0xc19;
const CSR_HPMCOUNTER26: u16 = 0xc1a;
const CSR_HPMCOUNTER27: u16 = 0xc1b;
const CSR_HPMCOUNTER28: u16 = 0xc1c;
const CSR_HPMCOUNTER29: u16 = 0xc1d;
const CSR_HPMCOUNTER30: u16 = 0xc1e;
const CSR_HPMCOUNTER31: u16 = 0xc1f;
const CSR_VL: u16 = 0xc20;
const CSR_VTYPE: u16 = 0xc21;
const CSR_VLENB: u16 = 0xc22;
const CSR_SSTATUS: u16 = 0x100;
const CSR_SEDELEG: u16 = 0x102;
const CSR_SIDELEG: u16 = 0x103;
const CSR_SIE: u16 = 0x104;
const CSR_STVEC: u16 = 0x105;
const CSR_SCOUNTEREN: u16 = 0x106;
const CSR_SENVCFG: u16 = 0x10a;
const CSR_SSTATEEN0: u16 = 0x10c;
const CSR_SSTATEEN1: u16 = 0x10d;
const CSR_SSTATEEN2: u16 = 0x10e;
const CSR_SSTATEEN3: u16 = 0x10f;
const CSR_SCOUNTINHIBIT: u16 = 0x120;
const CSR_SSCRATCH: u16 = 0x140;
const CSR_SEPC: u16 = 0x141;
const CSR_SCAUSE: u16 = 0x142;
const CSR_STVAL: u16 = 0x143;
const CSR_SIP: u16 = 0x144;
const CSR_STIMECMP: u16 = 0x14d;
const CSR_SISELECT: u16 = 0x150;
const CSR_SIREG: u16 = 0x151;
const CSR_SIREG2: u16 = 0x152;
const CSR_SIREG3: u16 = 0x153;
const CSR_SIREG4: u16 = 0x155;
const CSR_SIREG5: u16 = 0x156;
const CSR_SIREG6: u16 = 0x157;
const CSR_STOPEI: u16 = 0x15c;
const CSR_SATP: u16 = 0x180;
const CSR_SRMCFG: u16 = 0x181;
const CSR_SCONTEXT: u16 = 0x5a8;
const CSR_VSSTATUS: u16 = 0x200;
const CSR_VSIE: u16 = 0x204;
const CSR_VSTVEC: u16 = 0x205;
const CSR_VSSCRATCH: u16 = 0x240;
const CSR_VSEPC: u16 = 0x241;
const CSR_VSCAUSE: u16 = 0x242;
const CSR_VSTVAL: u16 = 0x243;
const CSR_VSIP: u16 = 0x244;
const CSR_VSTIMECMP: u16 = 0x24d;
const CSR_VSISELECT: u16 = 0x250;
const CSR_VSIREG: u16 = 0x251;
const CSR_VSIREG2: u16 = 0x252;
const CSR_VSIREG3: u16 = 0x253;
const CSR_VSIREG4: u16 = 0x255;
const CSR_VSIREG5: u16 = 0x256;
const CSR_VSIREG6: u16 = 0x257;
const CSR_VSTOPEI: u16 = 0x25c;
const CSR_VSATP: u16 = 0x280;
const CSR_HSTATUS: u16 = 0x600;
const CSR_HEDELEG: u16 = 0x602;
const CSR_HIDELEG: u16 = 0x603;
const CSR_HIE: u16 = 0x604;
const CSR_HTIMEDELTA: u16 = 0x605;
const CSR_HCOUNTEREN: u16 = 0x606;
const CSR_HGEIE: u16 = 0x607;
const CSR_HVIEN: u16 = 0x608;
const CSR_HVICTL: u16 = 0x609;
const CSR_HENVCFG: u16 = 0x60a;
const CSR_HSTATEEN0: u16 = 0x60c;
const CSR_HSTATEEN1: u16 = 0x60d;
const CSR_HSTATEEN2: u16 = 0x60e;
const CSR_HSTATEEN3: u16 = 0x60f;
const CSR_HTVAL: u16 = 0x643;
const CSR_HIP: u16 = 0x644;
const CSR_HVIP: u16 = 0x645;
const CSR_HVIPRIO1: u16 = 0x646;
const CSR_HVIPRIO2: u16 = 0x647;
const CSR_HTINST: u16 = 0x64a;
const CSR_HGATP: u16 = 0x680;
const CSR_HCONTEXT: u16 = 0x6a8;
const CSR_HGEIP: u16 = 0xe12;
const CSR_VSTOPI: u16 = 0xeb0;
const CSR_SCOUNTOVF: u16 = 0xda0;
const CSR_STOPI: u16 = 0xdb0;
const CSR_UTVT: u16 = 0x7;
const CSR_UNXTI: u16 = 0x45;
const CSR_UINTSTATUS: u16 = 0x46;
const CSR_USCRATCHCSW: u16 = 0x48;
const CSR_USCRATCHCSWL: u16 = 0x49;
const CSR_STVT: u16 = 0x107;
const CSR_SNXTI: u16 = 0x145;
const CSR_SINTSTATUS: u16 = 0x146;
const CSR_SSCRATCHCSW: u16 = 0x148;
const CSR_SSCRATCHCSWL: u16 = 0x149;
const CSR_MTVT: u16 = 0x307;
const CSR_MNXTI: u16 = 0x345;
const CSR_MINTSTATUS: u16 = 0x346;
const CSR_MSCRATCHCSW: u16 = 0x348;
const CSR_MSCRATCHCSWL: u16 = 0x349;
const CSR_MSTATUS: u16 = 0x300;
const CSR_MISA: u16 = 0x301;
const CSR_MEDELEG: u16 = 0x302;
const CSR_MIDELEG: u16 = 0x303;
const CSR_MIE: u16 = 0x304;
const CSR_MTVEC: u16 = 0x305;
const CSR_MCOUNTEREN: u16 = 0x306;
const CSR_MVIEN: u16 = 0x308;
const CSR_MVIP: u16 = 0x309;
const CSR_MENVCFG: u16 = 0x30a;
const CSR_MSTATEEN0: u16 = 0x30c;
const CSR_MSTATEEN1: u16 = 0x30d;
const CSR_MSTATEEN2: u16 = 0x30e;
const CSR_MSTATEEN3: u16 = 0x30f;
const CSR_MCOUNTINHIBIT: u16 = 0x320;
const CSR_MSCRATCH: u16 = 0x340;
const CSR_MEPC: u16 = 0x341;
const CSR_MCAUSE: u16 = 0x342;
const CSR_MTVAL: u16 = 0x343;
const CSR_MIP: u16 = 0x344;
const CSR_MTINST: u16 = 0x34a;
const CSR_MTVAL2: u16 = 0x34b;
const CSR_MISELECT: u16 = 0x350;
const CSR_MIREG: u16 = 0x351;
const CSR_MIREG2: u16 = 0x352;
const CSR_MIREG3: u16 = 0x353;
const CSR_MIREG4: u16 = 0x355;
const CSR_MIREG5: u16 = 0x356;
const CSR_MIREG6: u16 = 0x357;
const CSR_MTOPEI: u16 = 0x35c;
const CSR_PMPCFG0: u16 = 0x3a0;
const CSR_PMPCFG1: u16 = 0x3a1;
const CSR_PMPCFG2: u16 = 0x3a2;
const CSR_PMPCFG3: u16 = 0x3a3;
const CSR_PMPCFG4: u16 = 0x3a4;
const CSR_PMPCFG5: u16 = 0x3a5;
const CSR_PMPCFG6: u16 = 0x3a6;
const CSR_PMPCFG7: u16 = 0x3a7;
const CSR_PMPCFG8: u16 = 0x3a8;
const CSR_PMPCFG9: u16 = 0x3a9;
const CSR_PMPCFG10: u16 = 0x3aa;
const CSR_PMPCFG11: u16 = 0x3ab;
const CSR_PMPCFG12: u16 = 0x3ac;
const CSR_PMPCFG13: u16 = 0x3ad;
const CSR_PMPCFG14: u16 = 0x3ae;
const CSR_PMPCFG15: u16 = 0x3af;
const CSR_PMPADDR0: u16 = 0x3b0;
const CSR_PMPADDR1: u16 = 0x3b1;
const CSR_PMPADDR2: u16 = 0x3b2;
const CSR_PMPADDR3: u16 = 0x3b3;
const CSR_PMPADDR4: u16 = 0x3b4;
const CSR_PMPADDR5: u16 = 0x3b5;
const CSR_PMPADDR6: u16 = 0x3b6;
const CSR_PMPADDR7: u16 = 0x3b7;
const CSR_PMPADDR8: u16 = 0x3b8;
const CSR_PMPADDR9: u16 = 0x3b9;
const CSR_PMPADDR10: u16 = 0x3ba;
const CSR_PMPADDR11: u16 = 0x3bb;
const CSR_PMPADDR12: u16 = 0x3bc;
const CSR_PMPADDR13: u16 = 0x3bd;
const CSR_PMPADDR14: u16 = 0x3be;
const CSR_PMPADDR15: u16 = 0x3bf;
const CSR_PMPADDR16: u16 = 0x3c0;
const CSR_PMPADDR17: u16 = 0x3c1;
const CSR_PMPADDR18: u16 = 0x3c2;
const CSR_PMPADDR19: u16 = 0x3c3;
const CSR_PMPADDR20: u16 = 0x3c4;
const CSR_PMPADDR21: u16 = 0x3c5;
const CSR_PMPADDR22: u16 = 0x3c6;
const CSR_PMPADDR23: u16 = 0x3c7;
const CSR_PMPADDR24: u16 = 0x3c8;
const CSR_PMPADDR25: u16 = 0x3c9;
const CSR_PMPADDR26: u16 = 0x3ca;
const CSR_PMPADDR27: u16 = 0x3cb;
const CSR_PMPADDR28: u16 = 0x3cc;
const CSR_PMPADDR29: u16 = 0x3cd;
const CSR_PMPADDR30: u16 = 0x3ce;
const CSR_PMPADDR31: u16 = 0x3cf;
const CSR_PMPADDR32: u16 = 0x3d0;
const CSR_PMPADDR33: u16 = 0x3d1;
const CSR_PMPADDR34: u16 = 0x3d2;
const CSR_PMPADDR35: u16 = 0x3d3;
const CSR_PMPADDR36: u16 = 0x3d4;
const CSR_PMPADDR37: u16 = 0x3d5;
const CSR_PMPADDR38: u16 = 0x3d6;
const CSR_PMPADDR39: u16 = 0x3d7;
const CSR_PMPADDR40: u16 = 0x3d8;
const CSR_PMPADDR41: u16 = 0x3d9;
const CSR_PMPADDR42: u16 = 0x3da;
const CSR_PMPADDR43: u16 = 0x3db;
const CSR_PMPADDR44: u16 = 0x3dc;
const CSR_PMPADDR45: u16 = 0x3dd;
const CSR_PMPADDR46: u16 = 0x3de;
const CSR_PMPADDR47: u16 = 0x3df;
const CSR_PMPADDR48: u16 = 0x3e0;
const CSR_PMPADDR49: u16 = 0x3e1;
const CSR_PMPADDR50: u16 = 0x3e2;
const CSR_PMPADDR51: u16 = 0x3e3;
const CSR_PMPADDR52: u16 = 0x3e4;
const CSR_PMPADDR53: u16 = 0x3e5;
const CSR_PMPADDR54: u16 = 0x3e6;
const CSR_PMPADDR55: u16 = 0x3e7;
const CSR_PMPADDR56: u16 = 0x3e8;
const CSR_PMPADDR57: u16 = 0x3e9;
const CSR_PMPADDR58: u16 = 0x3ea;
const CSR_PMPADDR59: u16 = 0x3eb;
const CSR_PMPADDR60: u16 = 0x3ec;
const CSR_PMPADDR61: u16 = 0x3ed;
const CSR_PMPADDR62: u16 = 0x3ee;
const CSR_PMPADDR63: u16 = 0x3ef;
const CSR_MSECCFG: u16 = 0x747;
const CSR_TSELECT: u16 = 0x7a0;
const CSR_TDATA1: u16 = 0x7a1;
const CSR_TDATA2: u16 = 0x7a2;
const CSR_TDATA3: u16 = 0x7a3;
const CSR_TINFO: u16 = 0x7a4;
const CSR_TCONTROL: u16 = 0x7a5;
const CSR_MCONTEXT: u16 = 0x7a8;
const CSR_MSCONTEXT: u16 = 0x7aa;
const CSR_DCSR: u16 = 0x7b0;
const CSR_DPC: u16 = 0x7b1;
const CSR_DSCRATCH0: u16 = 0x7b2;
const CSR_DSCRATCH1: u16 = 0x7b3;
const CSR_MCYCLE: u16 = 0xb00;
const CSR_MINSTRET: u16 = 0xb02;
const CSR_MHPMCOUNTER3: u16 = 0xb03;
const CSR_MHPMCOUNTER4: u16 = 0xb04;
const CSR_MHPMCOUNTER5: u16 = 0xb05;
const CSR_MHPMCOUNTER6: u16 = 0xb06;
const CSR_MHPMCOUNTER7: u16 = 0xb07;
const CSR_MHPMCOUNTER8: u16 = 0xb08;
const CSR_MHPMCOUNTER9: u16 = 0xb09;
const CSR_MHPMCOUNTER10: u16 = 0xb0a;
const CSR_MHPMCOUNTER11: u16 = 0xb0b;
const CSR_MHPMCOUNTER12: u16 = 0xb0c;
const CSR_MHPMCOUNTER13: u16 = 0xb0d;
const CSR_MHPMCOUNTER14: u16 = 0xb0e;
const CSR_MHPMCOUNTER15: u16 = 0xb0f;
const CSR_MHPMCOUNTER16: u16 = 0xb10;
const CSR_MHPMCOUNTER17: u16 = 0xb11;
const CSR_MHPMCOUNTER18: u16 = 0xb12;
const CSR_MHPMCOUNTER19: u16 = 0xb13;
const CSR_MHPMCOUNTER20: u16 = 0xb14;
const CSR_MHPMCOUNTER21: u16 = 0xb15;
const CSR_MHPMCOUNTER22: u16 = 0xb16;
const CSR_MHPMCOUNTER23: u16 = 0xb17;
const CSR_MHPMCOUNTER24: u16 = 0xb18;
const CSR_MHPMCOUNTER25: u16 = 0xb19;
const CSR_MHPMCOUNTER26: u16 = 0xb1a;
const CSR_MHPMCOUNTER27: u16 = 0xb1b;
const CSR_MHPMCOUNTER28: u16 = 0xb1c;
const CSR_MHPMCOUNTER29: u16 = 0xb1d;
const CSR_MHPMCOUNTER30: u16 = 0xb1e;
const CSR_MHPMCOUNTER31: u16 = 0xb1f;
const CSR_MCYCLECFG: u16 = 0x321;
const CSR_MINSTRETCFG: u16 = 0x322;
const CSR_MHPMEVENT3: u16 = 0x323;
const CSR_MHPMEVENT4: u16 = 0x324;
const CSR_MHPMEVENT5: u16 = 0x325;
const CSR_MHPMEVENT6: u16 = 0x326;
const CSR_MHPMEVENT7: u16 = 0x327;
const CSR_MHPMEVENT8: u16 = 0x328;
const CSR_MHPMEVENT9: u16 = 0x329;
const CSR_MHPMEVENT10: u16 = 0x32a;
const CSR_MHPMEVENT11: u16 = 0x32b;
const CSR_MHPMEVENT12: u16 = 0x32c;
const CSR_MHPMEVENT13: u16 = 0x32d;
const CSR_MHPMEVENT14: u16 = 0x32e;
const CSR_MHPMEVENT15: u16 = 0x32f;
const CSR_MHPMEVENT16: u16 = 0x330;
const CSR_MHPMEVENT17: u16 = 0x331;
const CSR_MHPMEVENT18: u16 = 0x332;
const CSR_MHPMEVENT19: u16 = 0x333;
const CSR_MHPMEVENT20: u16 = 0x334;
const CSR_MHPMEVENT21: u16 = 0x335;
const CSR_MHPMEVENT22: u16 = 0x336;
const CSR_MHPMEVENT23: u16 = 0x337;
const CSR_MHPMEVENT24: u16 = 0x338;
const CSR_MHPMEVENT25: u16 = 0x339;
const CSR_MHPMEVENT26: u16 = 0x33a;
const CSR_MHPMEVENT27: u16 = 0x33b;
const CSR_MHPMEVENT28: u16 = 0x33c;
const CSR_MHPMEVENT29: u16 = 0x33d;
const CSR_MHPMEVENT30: u16 = 0x33e;
const CSR_MHPMEVENT31: u16 = 0x33f;
const CSR_MVENDORID: u16 = 0xf11;
const CSR_MARCHID: u16 = 0xf12;
const CSR_MIMPID: u16 = 0xf13;
const CSR_MHARTID: u16 = 0xf14;
const CSR_MCONFIGPTR: u16 = 0xf15;
const CSR_MTOPI: u16 = 0xfb0;
const CSR_SIEH: u16 = 0x114;
const CSR_SIPH: u16 = 0x154;
const CSR_STIMECMPH: u16 = 0x15d;
const CSR_VSIEH: u16 = 0x214;
const CSR_VSIPH: u16 = 0x254;
const CSR_VSTIMECMPH: u16 = 0x25d;
const CSR_HTIMEDELTAH: u16 = 0x615;
const CSR_HIDELEGH: u16 = 0x613;
const CSR_HVIENH: u16 = 0x618;
const CSR_HENVCFGH: u16 = 0x61a;
const CSR_HVIPH: u16 = 0x655;
const CSR_HVIPRIO1H: u16 = 0x656;
const CSR_HVIPRIO2H: u16 = 0x657;
const CSR_HSTATEEN0H: u16 = 0x61c;
const CSR_HSTATEEN1H: u16 = 0x61d;
const CSR_HSTATEEN2H: u16 = 0x61e;
const CSR_HSTATEEN3H: u16 = 0x61f;
const CSR_CYCLEH: u16 = 0xc80;
const CSR_TIMEH: u16 = 0xc81;
const CSR_INSTRETH: u16 = 0xc82;
const CSR_HPMCOUNTER3H: u16 = 0xc83;
const CSR_HPMCOUNTER4H: u16 = 0xc84;
const CSR_HPMCOUNTER5H: u16 = 0xc85;
const CSR_HPMCOUNTER6H: u16 = 0xc86;
const CSR_HPMCOUNTER7H: u16 = 0xc87;
const CSR_HPMCOUNTER8H: u16 = 0xc88;
const CSR_HPMCOUNTER9H: u16 = 0xc89;
const CSR_HPMCOUNTER10H: u16 = 0xc8a;
const CSR_HPMCOUNTER11H: u16 = 0xc8b;
const CSR_HPMCOUNTER12H: u16 = 0xc8c;
const CSR_HPMCOUNTER13H: u16 = 0xc8d;
const CSR_HPMCOUNTER14H: u16 = 0xc8e;
const CSR_HPMCOUNTER15H: u16 = 0xc8f;
const CSR_HPMCOUNTER16H: u16 = 0xc90;
const CSR_HPMCOUNTER17H: u16 = 0xc91;
const CSR_HPMCOUNTER18H: u16 = 0xc92;
const CSR_HPMCOUNTER19H: u16 = 0xc93;
const CSR_HPMCOUNTER20H: u16 = 0xc94;
const CSR_HPMCOUNTER21H: u16 = 0xc95;
const CSR_HPMCOUNTER22H: u16 = 0xc96;
const CSR_HPMCOUNTER23H: u16 = 0xc97;
const CSR_HPMCOUNTER24H: u16 = 0xc98;
const CSR_HPMCOUNTER25H: u16 = 0xc99;
const CSR_HPMCOUNTER26H: u16 = 0xc9a;
const CSR_HPMCOUNTER27H: u16 = 0xc9b;
const CSR_HPMCOUNTER28H: u16 = 0xc9c;
const CSR_HPMCOUNTER29H: u16 = 0xc9d;
const CSR_HPMCOUNTER30H: u16 = 0xc9e;
const CSR_HPMCOUNTER31H: u16 = 0xc9f;
const CSR_MSTATUSH: u16 = 0x310;
const CSR_MIDELEGH: u16 = 0x313;
const CSR_MIEH: u16 = 0x314;
const CSR_MVIENH: u16 = 0x318;
const CSR_MVIPH: u16 = 0x319;
const CSR_MENVCFGH: u16 = 0x31a;
const CSR_MSTATEEN0H: u16 = 0x31c;
const CSR_MSTATEEN1H: u16 = 0x31d;
const CSR_MSTATEEN2H: u16 = 0x31e;
const CSR_MSTATEEN3H: u16 = 0x31f;
const CSR_MIPH: u16 = 0x354;
const CSR_MCYCLECFGH: u16 = 0x721;
const CSR_MINSTRETCFGH: u16 = 0x722;
const CSR_MHPMEVENT3H: u16 = 0x723;
const CSR_MHPMEVENT4H: u16 = 0x724;
const CSR_MHPMEVENT5H: u16 = 0x725;
const CSR_MHPMEVENT6H: u16 = 0x726;
const CSR_MHPMEVENT7H: u16 = 0x727;
const CSR_MHPMEVENT8H: u16 = 0x728;
const CSR_MHPMEVENT9H: u16 = 0x729;
const CSR_MHPMEVENT10H: u16 = 0x72a;
const CSR_MHPMEVENT11H: u16 = 0x72b;
const CSR_MHPMEVENT12H: u16 = 0x72c;
const CSR_MHPMEVENT13H: u16 = 0x72d;
const CSR_MHPMEVENT14H: u16 = 0x72e;
const CSR_MHPMEVENT15H: u16 = 0x72f;
const CSR_MHPMEVENT16H: u16 = 0x730;
const CSR_MHPMEVENT17H: u16 = 0x731;
const CSR_MHPMEVENT18H: u16 = 0x732;
const CSR_MHPMEVENT19H: u16 = 0x733;
const CSR_MHPMEVENT20H: u16 = 0x734;
const CSR_MHPMEVENT21H: u16 = 0x735;
const CSR_MHPMEVENT22H: u16 = 0x736;
const CSR_MHPMEVENT23H: u16 = 0x737;
const CSR_MHPMEVENT24H: u16 = 0x738;
const CSR_MHPMEVENT25H: u16 = 0x739;
const CSR_MHPMEVENT26H: u16 = 0x73a;
const CSR_MHPMEVENT27H: u16 = 0x73b;
const CSR_MHPMEVENT28H: u16 = 0x73c;
const CSR_MHPMEVENT29H: u16 = 0x73d;
const CSR_MHPMEVENT30H: u16 = 0x73e;
const CSR_MHPMEVENT31H: u16 = 0x73f;
const CSR_MNSCRATCH: u16 = 0x740;
const CSR_MNEPC: u16 = 0x741;
const CSR_MNCAUSE: u16 = 0x742;
const CSR_MNSTATUS: u16 = 0x744;
const CSR_MSECCFGH: u16 = 0x757;
const CSR_MCYCLEH: u16 = 0xb80;
const CSR_MINSTRETH: u16 = 0xb82;
const CSR_MHPMCOUNTER3H: u16 = 0xb83;
const CSR_MHPMCOUNTER4H: u16 = 0xb84;
const CSR_MHPMCOUNTER5H: u16 = 0xb85;
const CSR_MHPMCOUNTER6H: u16 = 0xb86;
const CSR_MHPMCOUNTER7H: u16 = 0xb87;
const CSR_MHPMCOUNTER8H: u16 = 0xb88;
const CSR_MHPMCOUNTER9H: u16 = 0xb89;
const CSR_MHPMCOUNTER10H: u16 = 0xb8a;
const CSR_MHPMCOUNTER11H: u16 = 0xb8b;
const CSR_MHPMCOUNTER12H: u16 = 0xb8c;
const CSR_MHPMCOUNTER13H: u16 = 0xb8d;
const CSR_MHPMCOUNTER14H: u16 = 0xb8e;
const CSR_MHPMCOUNTER15H: u16 = 0xb8f;
const CSR_MHPMCOUNTER16H: u16 = 0xb90;
const CSR_MHPMCOUNTER17H: u16 = 0xb91;
const CSR_MHPMCOUNTER18H: u16 = 0xb92;
const CSR_MHPMCOUNTER19H: u16 = 0xb93;
const CSR_MHPMCOUNTER20H: u16 = 0xb94;
const CSR_MHPMCOUNTER21H: u16 = 0xb95;
const CSR_MHPMCOUNTER22H: u16 = 0xb96;
const CSR_MHPMCOUNTER23H: u16 = 0xb97;
const CSR_MHPMCOUNTER24H: u16 = 0xb98;
const CSR_MHPMCOUNTER25H: u16 = 0xb99;
const CSR_MHPMCOUNTER26H: u16 = 0xb9a;
const CSR_MHPMCOUNTER27H: u16 = 0xb9b;
const CSR_MHPMCOUNTER28H: u16 = 0xb9c;
const CSR_MHPMCOUNTER29H: u16 = 0xb9d;
const CSR_MHPMCOUNTER30H: u16 = 0xb9e;
const CSR_MHPMCOUNTER31H: u16 = 0xb9f;
const CAUSE_MISALIGNED_FETCH: u8 = 0x0;
const CAUSE_FETCH_ACCESS: u8 = 0x1;
const CAUSE_ILLEGAL_INSTRUCTION: u8 = 0x2;
const CAUSE_BREAKPOINT: u8 = 0x3;
const CAUSE_MISALIGNED_LOAD: u8 = 0x4;
const CAUSE_LOAD_ACCESS: u8 = 0x5;
const CAUSE_MISALIGNED_STORE: u8 = 0x6;
const CAUSE_STORE_ACCESS: u8 = 0x7;
const CAUSE_USER_ECALL: u8 = 0x8;
const CAUSE_SUPERVISOR_ECALL: u8 = 0x9;
const CAUSE_VIRTUAL_SUPERVISOR_ECALL: u8 = 0xa;
const CAUSE_MACHINE_ECALL: u8 = 0xb;
const CAUSE_FETCH_PAGE_FAULT: u8 = 0xc;
const CAUSE_LOAD_PAGE_FAULT: u8 = 0xd;
const CAUSE_STORE_PAGE_FAULT: u8 = 0xf;
const CAUSE_SOFTWARE_CHECK_FAULT: u8 = 0x12;
const CAUSE_HARDWARE_ERROR_FAULT: u8 = 0x13;
const CAUSE_FETCH_GUEST_PAGE_FAULT: u8 = 0x14;
const CAUSE_LOAD_GUEST_PAGE_FAULT: u8 = 0x15;
const CAUSE_VIRTUAL_INSTRUCTION: u8 = 0x16;
const CAUSE_STORE_GUEST_PAGE_FAULT: u8 = 0x17;

pub struct DecoderStruct<'a>{


adress_begin :u64,

adress_instructions : &'a [u8],




memory: Vec<(u64,u64)>,

section_instr_size: u32,
regs: [u64; 33],

// faire les drapeaux




 carry : bool,
negative: bool,
zero : bool,
equal : bool ,
overflow: bool ,
greater_than : bool,
greater_than_equal : bool,
less_than_equal : bool,
less_than : bool,


adress_debut_fonction : u64,
adress_begin_section : u64,
actual_x : u16,
actual_y : u16,
data_video : Vec<u8>,
}

impl<'a> DecoderStruct<'_> {
 pub fn new(adressbegin: u64,adress_instructions : &'a [u8],memory : Vec<(u64,u64)>, adress_begin_section : u64) -> DecoderStruct{
            DecoderStruct {
                adress_begin: adressbegin,
                adress_instructions: adress_instructions,
                memory: memory,
                section_instr_size: 0,
                regs: [0;33],
                carry : false,
                negative : false,
                zero : false,
                equal : false,
                overflow: false,
                greater_than : false, 
                greater_than_equal : false,
                less_than_equal : false, 
                less_than : false,
                adress_debut_fonction: adressbegin,
                adress_begin_section: adress_begin_section,
                actual_x: 0,
                actual_y: 0,
                data_video: vec![0; 800 * 600 * 3],
            }
        }

fn clear_flags(&mut self) {

    self.carry  =  false;
    self.negative  =  false;
    self.zero  =  false;
    self.equal  =  false;
    self.overflow = false;
    self.greater_than  =  false; 
    self.less_than_equal  =  false; 
    self.less_than  =  false;


}
fn check_data(&mut self ,data : i64,  data2 : i64) {



    if data < 0 || data2 < 0 {

        self.negative = true;
    }

    if data & 1 << 63  == 1{

        // carry flag 

        self.carry = true;
        self.overflow = true;

    }

    if data2 & 1 << 63 == 1 {

        self.carry = true;
        self.overflow = true;
    }


    if data == 0 || data2 == 0 {

        self.zero = true;

    }


    if data == data2 {

        self.equal = true;

    }

    if data > data2 {
        self.greater_than = true;

    }
    if data >= data2 {

        self.greater_than_equal = true;
    }
    if data <= data2 {

        self.less_than_equal = true;
    }
    if data < data2 {

        self. less_than = true;
    }

}


fn check_data_unsigned(&mut self ,data : u64,  data2 : u64) {



    if data < 0 || data2 < 0 {

        self.negative = true;
    }

    if data & 1 << 63  == 1{

        // carry flag 

        self.carry = true;
        self.overflow = true;

    }

    if data2 & 1 << 63 == 1 {

        self.carry = true;
        self.overflow = true;
    }


    if data == 0 || data2 == 0 {

        self.zero = true;

    }


    if data == data2 {

        self.equal = true;

    }

    if data > data2 {
        self.greater_than = true;

    }
    if data >= data2 {

        self.greater_than_equal = true;
    }
    if data <= data2 {

        self.less_than_equal = true;
    }
    if data < data2 {

        self. less_than = true;
    }

}
fn copy(&mut self) {

let mut status = 0 as usize;
let mut adress : u64 = self.adress_begin_section as u64;
let mut i :  usize = 0;

println!("{}",self.adress_instructions.len());
while true {

if status >= self.adress_instructions.len() {

    break;
}


let tup  = ((adress as usize +i) as u64,((self.adress_instructions[status] as u32)  | (self.adress_instructions[status +1] as u32) << 8  | ( self.adress_instructions[status +2] as u32 ) << 16  | (self.adress_instructions[status +3] as u32) << 24) as u64);

self.memory.push(tup)  ;


status += 4;
i += 1;
self.section_instr_size+=1;
adress += 3;
}




}

fn memory_write_char(&mut self,addr: u64,data : u64) {


let tup = ((addr as u64), (data));
self.memory.push(tup)  ;
self.video(addr,data);
}

fn memory_write_short(&mut self,addr: u64,data : u64) {


    let tup = ((addr as u64), (data));
    self.memory.push(tup)  ;
    self.video(addr,data);

    }
    
fn memory_write_long(&mut self,addr: u64,data : u64) {


    let tup = ((addr as u64), (data));
   self.memory.push(tup)  ;
   self.video(addr,data);

}
        
/// trouver un index avec l'adresse
fn find_data_with_addr(&mut self,adress_ :u64) ->  Option<u64> {


for i in 0..self.memory.len() {

let tmp = self.memory[i];


if adress_ ==  tmp.0 {

return Some(i as u64);
}


}

return Some(u64::MAX);
}

fn debug_gaspard(&mut self,x: &str ) {


println!("{}",x);
console_log!("{}",x);


add_instr_to_dom(x);




}




fn extract_instr_16 (&mut self,instr :u32) -> (u16,u16) {


let half :u16 = (instr & 0b1111111111111111).try_into().unwrap() ;
let upper : u16 = (instr >> 16  & 0b1111111111111111).try_into().unwrap() ;


return (half,upper);

}

/// fonction qui permet d'émuler les instructions sur 32 bits et16 bits si true alors rvc sinon normal
fn normal_scan(&mut self,instr :u32) -> bool {
// TODO




/* Automatically generated by parse_opcodes */
if instr &  MASK_ADD ==  MATCH_ADD { 
    
    self.debug_gaspard("ADD ");
    



    // recuperer les reigstrers et faire l'opération
   



   let rd = instr >> 7 & 0b1111;

   let rs1 = instr >> 15  & 0b1111;
   let rs2 = instr >> 20 & 0b1111;
   self.clear_flags(); 


   self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);



   self.regs[rd as usize] = self.regs[rs1 as usize] + self.regs[rs2 as usize];




   
    return false;
    }if instr &  MASK_ADD16 ==  MATCH_ADD16 { 
    
    self.debug_gaspard("ADD16 ");
   return false;
    }if instr &  MASK_ADD32 ==  MATCH_ADD32 { 
    
    self.debug_gaspard("ADD32 ");
   return false;
    }if instr &  MASK_ADD64 ==  MATCH_ADD64 { 
    
    self.debug_gaspard("ADD64 ");
   return false;
    }if instr &  MASK_ADD8 ==  MATCH_ADD8 { 
    
    self.debug_gaspard("ADD8 ");
   return false;
    }if instr &  MASK_ADD_UW ==  MATCH_ADD_UW { 
    
    self.debug_gaspard("ADD_UW ");
   return false;
    }if instr &  MASK_ADDD ==  MATCH_ADDD { 
    
    self.debug_gaspard("ADDD ");
   return false;
    }if instr &  MASK_ADDI ==  MATCH_ADDI { 
    
    self.debug_gaspard("ADDI ");


        let rd = instr >> 7 & 0b1111;

        let rs1 = instr >> 15  & 0b1111;
        let imm = instr >> 20;
        self.clear_flags(); 


        self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);
        self.regs[rd as usize] = self.regs[rs1 as usize] + imm as u64;


   return false;
    }if instr &  MASK_ADDID ==  MATCH_ADDID { 
    
    self.debug_gaspard("ADDID ");
   return false;
    }if instr &  MASK_ADDIW ==  MATCH_ADDIW { 
    
    self.debug_gaspard("ADDIW ");
   return false;
    }if instr &  MASK_ADDW ==  MATCH_ADDW { 
    
    self.debug_gaspard("ADDW ");
   return false;
    }if instr &  MASK_AES32DSI ==  MATCH_AES32DSI { 
    
    self.debug_gaspard("AES32DSI ");
   return false;
    }if instr &  MASK_AES32DSMI ==  MATCH_AES32DSMI { 
    
    self.debug_gaspard("AES32DSMI ");
   return false;
    }if instr &  MASK_AES32ESI ==  MATCH_AES32ESI { 
    
    self.debug_gaspard("AES32ESI ");
   return false;
    }if instr &  MASK_AES32ESMI ==  MATCH_AES32ESMI { 
    
    self.debug_gaspard("AES32ESMI ");
   return false;
    }if instr &  MASK_AES64DS ==  MATCH_AES64DS { 
    
    self.debug_gaspard("AES64DS ");
   return false;
    }if instr &  MASK_AES64DSM ==  MATCH_AES64DSM { 
    
    self.debug_gaspard("AES64DSM ");
   return false;
    }if instr &  MASK_AES64ES ==  MATCH_AES64ES { 
    
    self.debug_gaspard("AES64ES ");
   return false;
    }if instr &  MASK_AES64ESM ==  MATCH_AES64ESM { 
    
    self.debug_gaspard("AES64ESM ");
   return false;
    }if instr &  MASK_AES64IM ==  MATCH_AES64IM { 
    
    self.debug_gaspard("AES64IM ");
   return false;
    }if instr &  MASK_AES64KS1I ==  MATCH_AES64KS1I { 
    
    self.debug_gaspard("AES64KS1I ");
   return false;
    }if instr &  MASK_AES64KS2 ==  MATCH_AES64KS2 { 
    
    self.debug_gaspard("AES64KS2 ");
   return false;
    }if instr &  MASK_AMOADD_B ==  MATCH_AMOADD_B { 
    
    self.debug_gaspard("AMOADD_B ");
   return false;
    }if instr &  MASK_AMOADD_D ==  MATCH_AMOADD_D { 
    
    self.debug_gaspard("AMOADD_D ");
   return false;
    }if instr &  MASK_AMOADD_H ==  MATCH_AMOADD_H { 
    
    self.debug_gaspard("AMOADD_H ");
   return false;
    }if instr &  MASK_AMOADD_W ==  MATCH_AMOADD_W { 
    
    self.debug_gaspard("AMOADD_W ");
   return false;
    }if instr &  MASK_AMOAND_B ==  MATCH_AMOAND_B { 
    
    self.debug_gaspard("AMOAND_B ");
   return false;
    }if instr &  MASK_AMOAND_D ==  MATCH_AMOAND_D { 
    
    self.debug_gaspard("AMOAND_D ");
   return false;
    }if instr &  MASK_AMOAND_H ==  MATCH_AMOAND_H { 
    
    self.debug_gaspard("AMOAND_H ");
   return false;
    }if instr &  MASK_AMOAND_W ==  MATCH_AMOAND_W { 
    
    self.debug_gaspard("AMOAND_W ");
   return false;
    }if instr &  MASK_AMOCAS_B ==  MATCH_AMOCAS_B { 
    
    self.debug_gaspard("AMOCAS_B ");
   return false;
    }if instr &  MASK_AMOCAS_D ==  MATCH_AMOCAS_D { 
    
    self.debug_gaspard("AMOCAS_D ");
   return false;
    }if instr &  MASK_AMOCAS_H ==  MATCH_AMOCAS_H { 
    
    self.debug_gaspard("AMOCAS_H ");
   return false;
    }if instr &  MASK_AMOCAS_Q ==  MATCH_AMOCAS_Q { 
    
    self.debug_gaspard("AMOCAS_Q ");
   return false;
    }if instr &  MASK_AMOCAS_W ==  MATCH_AMOCAS_W { 
    
    self.debug_gaspard("AMOCAS_W ");
   return false;
    }if instr &  MASK_AMOMAX_B ==  MATCH_AMOMAX_B { 
    
    self.debug_gaspard("AMOMAX_B ");
   return false;
    }if instr &  MASK_AMOMAX_D ==  MATCH_AMOMAX_D { 
    
    self.debug_gaspard("AMOMAX_D ");
   return false;
    }if instr &  MASK_AMOMAX_H ==  MATCH_AMOMAX_H { 
    
    self.debug_gaspard("AMOMAX_H ");
   return false;
    }if instr &  MASK_AMOMAX_W ==  MATCH_AMOMAX_W { 
    
    self.debug_gaspard("AMOMAX_W ");
   return false;
    }if instr &  MASK_AMOMAXU_B ==  MATCH_AMOMAXU_B { 
    
    self.debug_gaspard("AMOMAXU_B ");
   return false;
    }if instr &  MASK_AMOMAXU_D ==  MATCH_AMOMAXU_D { 
    
    self.debug_gaspard("AMOMAXU_D ");
   return false;
    }if instr &  MASK_AMOMAXU_H ==  MATCH_AMOMAXU_H { 
    
    self.debug_gaspard("AMOMAXU_H ");
   return false;
    }if instr &  MASK_AMOMAXU_W ==  MATCH_AMOMAXU_W { 
    
    self.debug_gaspard("AMOMAXU_W ");
   return false;
    }if instr &  MASK_AMOMIN_B ==  MATCH_AMOMIN_B { 
    
    self.debug_gaspard("AMOMIN_B ");
   return false;
    }if instr &  MASK_AMOMIN_D ==  MATCH_AMOMIN_D { 
    
    self.debug_gaspard("AMOMIN_D ");
   return false;
    }if instr &  MASK_AMOMIN_H ==  MATCH_AMOMIN_H { 
    
    self.debug_gaspard("AMOMIN_H ");
   return false;
    }if instr &  MASK_AMOMIN_W ==  MATCH_AMOMIN_W { 
    
    self.debug_gaspard("AMOMIN_W ");
   return false;
    }if instr &  MASK_AMOMINU_B ==  MATCH_AMOMINU_B { 
    
    self.debug_gaspard("AMOMINU_B ");
   return false;
    }if instr &  MASK_AMOMINU_D ==  MATCH_AMOMINU_D { 
    
    self.debug_gaspard("AMOMINU_D ");
   return false;
    }if instr &  MASK_AMOMINU_H ==  MATCH_AMOMINU_H { 
    
    self.debug_gaspard("AMOMINU_H ");
   return false;
    }if instr &  MASK_AMOMINU_W ==  MATCH_AMOMINU_W { 
    
    self.debug_gaspard("AMOMINU_W ");
   return false;
    }if instr &  MASK_AMOOR_B ==  MATCH_AMOOR_B { 
    
    self.debug_gaspard("AMOOR_B ");
   return false;
    }if instr &  MASK_AMOOR_D ==  MATCH_AMOOR_D { 
    
    self.debug_gaspard("AMOOR_D ");
   return false;
    }if instr &  MASK_AMOOR_H ==  MATCH_AMOOR_H { 
    
    self.debug_gaspard("AMOOR_H ");
   return false;
    }if instr &  MASK_AMOOR_W ==  MATCH_AMOOR_W { 
    
    self.debug_gaspard("AMOOR_W ");
   return false;
    }if instr &  MASK_AMOSWAP_B ==  MATCH_AMOSWAP_B { 
    
    self.debug_gaspard("AMOSWAP_B ");
   return false;
    }if instr &  MASK_AMOSWAP_D ==  MATCH_AMOSWAP_D { 
    
    self.debug_gaspard("AMOSWAP_D ");
   return false;
    }if instr &  MASK_AMOSWAP_H ==  MATCH_AMOSWAP_H { 
    
    self.debug_gaspard("AMOSWAP_H ");
   return false;
    }if instr &  MASK_AMOSWAP_W ==  MATCH_AMOSWAP_W { 
    
    self.debug_gaspard("AMOSWAP_W ");
   return false;
    }if instr &  MASK_AMOXOR_B ==  MATCH_AMOXOR_B { 
    
    self.debug_gaspard("AMOXOR_B ");
   return false;
    }if instr &  MASK_AMOXOR_D ==  MATCH_AMOXOR_D { 
    
    self.debug_gaspard("AMOXOR_D ");
   return false;
    }if instr &  MASK_AMOXOR_H ==  MATCH_AMOXOR_H { 
    
    self.debug_gaspard("AMOXOR_H ");
   return false;
    }if instr &  MASK_AMOXOR_W ==  MATCH_AMOXOR_W { 
    
    self.debug_gaspard("AMOXOR_W ");
   return false;
    }if instr &  MASK_AND ==  MATCH_AND { 
    
    self.debug_gaspard("AND ");


    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
self.regs[rd as usize] = self.regs[rs1 as usize] & self.regs[rs2 as usize];






   return false;
    }if instr &  MASK_ANDI ==  MATCH_ANDI { 
    
    self.debug_gaspard("ANDI ");



    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let imm = instr >> 20;
    self.clear_flags(); 


    self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);



    self.regs[rd as usize] = self.regs[rs1 as usize] & imm as u64;




   return false;
    }if instr &  MASK_ANDN ==  MATCH_ANDN { 
    
    self.debug_gaspard("ANDN ");
   return false;
    }if instr &  MASK_AUIPC ==  MATCH_AUIPC { 
    
    self.debug_gaspard("AUIPC ");


    self.clear_flags();


    let imm : u32 = instr >> 12 ;
    let rd : u32 = instr>>7 & 0b11111;
    self.regs[rd as usize] = (imm << 12) as u64; 
    self.regs[32] = self.regs[rd as usize]; 


   return false;
    }if instr &  MASK_AVE ==  MATCH_AVE { 
    
    self.debug_gaspard("AVE ");
   return false;
    }if instr &  MASK_BCLR ==  MATCH_BCLR { 
    
    self.debug_gaspard("BCLR ");
   return false;
    }if instr &  MASK_BCLRI ==  MATCH_BCLRI { 
    
    self.debug_gaspard("BCLRI ");
   return false;
    }if instr &  MASK_BCLRI_RV32 ==  MATCH_BCLRI_RV32 { 
    
    self.debug_gaspard("BCLRI_RV32 ");
   return false;
    }if instr &  MASK_BCOMPRESS ==  MATCH_BCOMPRESS { 
    
    self.debug_gaspard("BCOMPRESS ");
   return false;
    }if instr &  MASK_BCOMPRESSW ==  MATCH_BCOMPRESSW { 
    
    self.debug_gaspard("BCOMPRESSW ");
   return false;
    }if instr &  MASK_BDECOMPRESS ==  MATCH_BDECOMPRESS { 
    
    self.debug_gaspard("BDECOMPRESS ");
   return false;
    }if instr &  MASK_BDECOMPRESSW ==  MATCH_BDECOMPRESSW { 
    
    self.debug_gaspard("BDECOMPRESSW ");
   return false;
    }if instr &  MASK_BEQ ==  MATCH_BEQ { 
    
    self.debug_gaspard("BEQ ");
    let IMM1 = instr >>7 & 0b11111;
    let IMM2 = instr >> 25 & 0b1111111;
    let first = IMM1 & 0b1; // 11 eme bits IMM[11]	
    let second = IMM1 >> 1 & 0b11110; // IMM[4:1]
    let third = IMM2 & 0b111111;	//imm[10:5]	
    let fourth = IMM2 >> 6 & 0b1; // IMM[12]
    let rs1 = instr >> 15 & 0b11111;
    let rs2 = instr >> 20 & 0b11111;
    
    //unsigned long imm = second | third << 4 |  first << 5 | fourth << 6; // a relire 
    //unsigned long imm = second | third << 6 |  first << 11 | fourth << 12; // a relire 
    let  imm = (second <<1 | third << 5 |  first << 11 | fourth << 12 ); // a relire faire l'extension 
    //unsigned long   imm = second | third << 6 |  first << 7 | fourth << 8; // a relire 
    //imm+=2;
    /*
     * OK, il faut maintenant "bourrer" le bit à partir du 20 ième bits en suivant la doc de riscv 
     * 0x1ffff represete un bourrage de 20 bits  = 32-12= 20
     * https://stackoverflow.com/questions/75048599/branch-riscv-instruction-offset-calculation-on-my-emulator/75048755#75048755
     * */
    let imm_final : i32  = (imm | 0x1FFFFF << 12) as i32; // on suite la doc qui me dit que je dois me positionner à partir de 12 et à partir de là bourrer sur 20 bits :
    
        println!("beq immediat final {:x}",imm_final);

        let rs1 = instr >> 15 & 0b1111;
        let rs2 = instr >> 20 & 0b1111;

    self.clear_flags();

    self.check_data_unsigned(self.regs[rs1 as usize], self.regs[rs2 as usize]); 

    if self.equal == true {


       // self.regs[32] = (self.regs[32]+imm_final as u64) ; // PAS SUR DE LA SOLUTION le signed extend n'a pas été fait );

        self.regs[32] = (self.regs[32].wrapping_add(imm_final as u64)as i64) .abs() as u64 -4;
        println!("final regs[32] {:x}",self.regs[32]);
    }



   return false;
    }if instr &  MASK_BEXT ==  MATCH_BEXT { 
    
    self.debug_gaspard("BEXT ");
   return false;
    }if instr &  MASK_BEXTI ==  MATCH_BEXTI { 
    
    self.debug_gaspard("BEXTI ");
   return false;
    }if instr &  MASK_BEXTI_RV32 ==  MATCH_BEXTI_RV32 { 
    
    self.debug_gaspard("BEXTI_RV32 ");
   return false;
    }if instr &  MASK_BFP ==  MATCH_BFP { 
    
    self.debug_gaspard("BFP ");
   return false;
    }if instr &  MASK_BFPW ==  MATCH_BFPW { 
    
    self.debug_gaspard("BFPW ");
   return false;
    }if instr &  MASK_BGE ==  MATCH_BGE { 
    
    self.debug_gaspard("BGE ");


    self. clear_flags();
        
    let IMM1 = instr >>7 & 0b11111;
    let IMM2 = instr >> 25 & 0b1111111;
    let first = IMM1 & 0b1; // 11 eme bits IMM[11]	
    let second = IMM1 >> 1 & 0b11110; // IMM[4:1]
    let third = IMM2 & 0b111111;	//imm[10:5]	
    let fourth = IMM2 >> 6 & 0b1; // IMM[12]
    let rs1 = instr >> 15 & 0b11111;
    let rs2 = instr >> 20 & 0b11111;
    
    //unsigned long imm = second | third << 4 |  first << 5 | fourth << 6; // a relire 
    //unsigned long imm = second | third << 6 |  first << 11 | fourth << 12; // a relire 
     let  imm = (second <<1 | third << 5 |  first << 11 | fourth << 12 ); // a relire faire l'extension 
    //unsigned long   imm = second | third << 6 |  first << 7 | fourth << 8; // a relire 
    //imm+=2;
    /*
     * OK, il faut maintenant "bourrer" le bit à partir du 20 ième bits en suivant la doc de riscv 
     * 0x1ffff represete un bourrage de 20 bits  = 32-12= 20
     * https://stackoverflow.com/questions/75048599/branch-riscv-instruction-offset-calculation-on-my-emulator/75048755#75048755
     * */
     let imm_final  = imm | 0x1FFFFF << 12; // on suite la doc qui me dit que je dois me positionner à partir de 12 et à partir de là bourrer sur 20 bits :
    //32-12=20 bits 0x1FFFFF represente 20 bits

    let rs1 = instr >> 15 & 0b1111;
    let rs2 = instr >> 20 & 0b1111;

            self.check_data(self.regs[rs1 as usize] as i64, self.regs[rs2 as usize] as i64);
    
    
    if self.greater_than == true {



        self.regs[32] = (self.regs[32].wrapping_add(imm_final as u64)as i64) .abs() as u64 -4;
        println!("final regs[32] {:x}",self.regs[32]);


    }






   return false;
    }if instr &  MASK_BGEU ==  MATCH_BGEU { 
    
    self.debug_gaspard("BGEU ");







    self. clear_flags();
        
    let IMM1 = instr >>7 & 0b11111;
    let IMM2 = instr >> 25 & 0b1111111;
    let first = IMM1 & 0b1; // 11 eme bits IMM[11]	
    let second = IMM1 >> 1 & 0b11110; // IMM[4:1]
    let third = IMM2 & 0b111111;	//imm[10:5]	
    let fourth = IMM2 >> 6 & 0b1; // IMM[12]
    let rs1 = instr >> 15 & 0b11111;
    let rs2 = instr >> 20 & 0b11111;
    
    //unsigned long imm = second | third << 4 |  first << 5 | fourth << 6; // a relire 
    //unsigned long imm = second | third << 6 |  first << 11 | fourth << 12; // a relire 
     let  imm = (second <<1 | third << 5 |  first << 11 | fourth << 12 ); // a relire faire l'extension 
    //unsigned long   imm = second | third << 6 |  first << 7 | fourth << 8; // a relire 
    //imm+=2;
    /*
     * OK, il faut maintenant "bourrer" le bit à partir du 20 ième bits en suivant la doc de riscv 
     * 0x1ffff represete un bourrage de 20 bits  = 32-12= 20
     * https://stackoverflow.com/questions/75048599/branch-riscv-instruction-offset-calculation-on-my-emulator/75048755#75048755
     * */
     let imm_final  = imm | 0x1FFFFF << 12; // on suite la doc qui me dit que je dois me positionner à partir de 12 et à partir de là bourrer sur 20 bits :
    //32-12=20 bits 0x1FFFFF represente 20 bits

    let rs1 = instr >> 15 & 0b1111;
    let rs2 = instr >> 20 & 0b1111;

            self.check_data(self.regs[rs1 as usize] as i64, self.regs[rs2 as usize] as i64);
    
    
    if self.greater_than == true {



        self.regs[32] = (self.regs[32].wrapping_add(imm_final as u64)as i64) .abs() as u64 -4;
        println!("final regs[32] {:x}",self.regs[32]);



    }




   return false;
    }if instr &  MASK_BINV ==  MATCH_BINV { 
    
    self.debug_gaspard("BINV ");
   return false;
    }if instr &  MASK_BINVI ==  MATCH_BINVI { 
    
    self.debug_gaspard("BINVI ");
   return false;
    }if instr &  MASK_BINVI_RV32 ==  MATCH_BINVI_RV32 { 
    
    self.debug_gaspard("BINVI_RV32 ");
   return false;
    }if instr &  MASK_BLT ==  MATCH_BLT { 
    
    self.debug_gaspard("BLT ");




   self. clear_flags();
        
    let IMM1 = instr >>7 & 0b11111;
    let IMM2 = instr >> 25 & 0b1111111;
    let first = IMM1 & 0b1; // 11 eme bits IMM[11]	
    let second = IMM1 >> 1 & 0b11110; // IMM[4:1]
    let third = IMM2 & 0b111111;	//imm[10:5]	
    let fourth = IMM2 >> 6 & 0b1; // IMM[12]
    let rs1 = instr >> 15 & 0b11111;
    let rs2 = instr >> 20 & 0b11111;
    
    //unsigned long imm = second | third << 4 |  first << 5 | fourth << 6; // a relire 
    //unsigned long imm = second | third << 6 |  first << 11 | fourth << 12; // a relire 
     let  imm = (second <<1 | third << 5 |  first << 11 | fourth << 12 ); // a relire faire l'extension 
    //unsigned long   imm = second | third << 6 |  first << 7 | fourth << 8; // a relire 
    //imm+=2;
    /*
     * OK, il faut maintenant "bourrer" le bit à partir du 20 ième bits en suivant la doc de riscv 
     * 0x1ffff represete un bourrage de 20 bits  = 32-12= 20
     * https://stackoverflow.com/questions/75048599/branch-riscv-instruction-offset-calculation-on-my-emulator/75048755#75048755
     * */
     let imm_final  = imm | 0x1FFFFF << 12; // on suite la doc qui me dit que je dois me positionner à partir de 12 et à partir de là bourrer sur 20 bits :
    //32-12=20 bits 0x1FFFFF represente 20 bits

    let rs1 = instr >> 15 & 0b1111;
    let rs2 = instr >> 20 & 0b1111;

            self.check_data(self.regs[rs1 as usize] as i64, self.regs[rs2 as usize] as i64);
    
    
    if self.less_than == true {



        self.regs[32] = (self.regs[32].wrapping_add(imm_final as u64)as i64) .abs() as u64 -4;
        println!("final regs[32] {:x}",self.regs[32]);



    }





   return false;
    }if instr &  MASK_BLTU ==  MATCH_BLTU { 
    
    self.debug_gaspard("BLTU ");






    self. clear_flags();
        
    let IMM1 = instr >>7 & 0b11111;
    let IMM2 = instr >> 25 & 0b1111111;
    let first = IMM1 & 0b1; // 11 eme bits IMM[11]	
    let second = IMM1 >> 1 & 0b11110; // IMM[4:1]
    let third = IMM2 & 0b111111;	//imm[10:5]	
    let fourth = IMM2 >> 6 & 0b1; // IMM[12]
    let rs1 = instr >> 15 & 0b11111;
    let rs2 = instr >> 20 & 0b11111;
    
    //unsigned long imm = second | third << 4 |  first << 5 | fourth << 6; // a relire 
    //unsigned long imm = second | third << 6 |  first << 11 | fourth << 12; // a relire 
     let  imm = (second <<1 | third << 5 |  first << 11 | fourth << 12 ); // a relire faire l'extension 
    //unsigned long   imm = second | third << 6 |  first << 7 | fourth << 8; // a relire 
    //imm+=2;
    /*
     * OK, il faut maintenant "bourrer" le bit à partir du 20 ième bits en suivant la doc de riscv 
     * 0x1ffff represete un bourrage de 20 bits  = 32-12= 20
     * https://stackoverflow.com/questions/75048599/branch-riscv-instruction-offset-calculation-on-my-emulator/75048755#75048755
     * */
     let imm_final  = imm | 0x1FFFFF << 12; // on suite la doc qui me dit que je dois me positionner à partir de 12 et à partir de là bourrer sur 20 bits :
    //32-12=20 bits 0x1FFFFF represente 20 bits

    let rs1 = instr >> 15 & 0b1111;
    let rs2 = instr >> 20 & 0b1111;

            self.check_data(self.regs[rs1 as usize] as i64, self.regs[rs2 as usize] as i64);
    
    
    if self.less_than == true {



        self.regs[32] = (self.regs[32].wrapping_add(imm_final as u64)as i64) .abs() as u64 -4;
        println!("final regs[32] {:x}",self.regs[32]);



    }







   return false;
    }if instr &  MASK_BMATFLIP ==  MATCH_BMATFLIP { 
    
    self.debug_gaspard("BMATFLIP ");
   return false;
    }if instr &  MASK_BMATOR ==  MATCH_BMATOR { 
    
    self.debug_gaspard("BMATOR ");
   return false;
    }if instr &  MASK_BMATXOR ==  MATCH_BMATXOR { 
    
    self.debug_gaspard("BMATXOR ");
   return false;
    }if instr &  MASK_BNE ==  MATCH_BNE { 
    
    self.debug_gaspard("BNE ");





    self.debug_gaspard("BGE ");


    self. clear_flags();
        
    let IMM1 = instr >>7 & 0b11111;
    let IMM2 = instr >> 25 & 0b1111111;
    let first = IMM1 & 0b1; // 11 eme bits IMM[11]	
    let second = IMM1 >> 1 & 0b11110; // IMM[4:1]
    let third = IMM2 & 0b111111;	//imm[10:5]	
    let fourth = IMM2 >> 6 & 0b1; // IMM[12]
    let rs1 = instr >> 15 & 0b11111;
    let rs2 = instr >> 20 & 0b11111;
    
    //unsigned long imm = second | third << 4 |  first << 5 | fourth << 6; // a relire 
    //unsigned long imm = second | third << 6 |  first << 11 | fourth << 12; // a relire 
     let  imm = (second <<1 | third << 5 |  first << 11 | fourth << 12 ); // a relire faire l'extension 
    //unsigned long   imm = second | third << 6 |  first << 7 | fourth << 8; // a relire 
    //imm+=2;
    /*
     * OK, il faut maintenant "bourrer" le bit à partir du 20 ième bits en suivant la doc de riscv 
     * 0x1ffff represete un bourrage de 20 bits  = 32-12= 20
     * https://stackoverflow.com/questions/75048599/branch-riscv-instruction-offset-calculation-on-my-emulator/75048755#75048755
     * */
     let imm_final  = imm | 0x1FFFFF << 12; // on suite la doc qui me dit que je dois me positionner à partir de 12 et à partir de là bourrer sur 20 bits :
    //32-12=20 bits 0x1FFFFF represente 20 bits

    let rs1 = instr >> 15 & 0b1111;
    let rs2 = instr >> 20 & 0b1111;

            self.check_data(self.regs[rs1 as usize] as i64, self.regs[rs2 as usize] as i64);
    
    
    if self.equal == false {



        self.regs[32] = (self.regs[32].wrapping_add(imm_final as u64)as i64) .abs() as u64 -4;
        println!("final regs[32] {:x}",self.regs[32]);



    }



   return false;
    }if instr &  MASK_BREV8 ==  MATCH_BREV8 { 
    
    self.debug_gaspard("BREV8 ");
   return false;
    }if instr &  MASK_BSET ==  MATCH_BSET { 
    
    self.debug_gaspard("BSET ");
   return false;
    }if instr &  MASK_BSETI ==  MATCH_BSETI { 
    
    self.debug_gaspard("BSETI ");
   return false;
    }if instr &  MASK_BSETI_RV32 ==  MATCH_BSETI_RV32 { 
    
    self.debug_gaspard("BSETI_RV32 ");
   return false;
    }if instr &  MASK_C_ADD ==  MATCH_C_ADD { 
    
    self.debug_gaspard("C_ADD ");
   return true;
    }if instr &  MASK_C_ADDI ==  MATCH_C_ADDI { 
    
    self.debug_gaspard("C_ADDI ");
   return true;
    }if instr &  MASK_C_ADDI16SP ==  MATCH_C_ADDI16SP { 
    
    self.debug_gaspard("C_ADDI16SP ");
    
    
    //let imm : i8 = EXTRACT_CITYPE_ADDI16SP_IMM(instr) as i8;
    
    
   return true;
    }if instr &  MASK_C_ADDI4SPN ==  MATCH_C_ADDI4SPN { 
    
    self.debug_gaspard("C_ADDI4SPN ");
   return true;
    }if instr &  MASK_C_ADDIW ==  MATCH_C_ADDIW { 
    
    self.debug_gaspard("C_ADDIW ");
   return true;
    }if instr &  MASK_C_ADDW ==  MATCH_C_ADDW { 
    
    self.debug_gaspard("C_ADDW ");
   return true;
    }if instr &  MASK_C_AND ==  MATCH_C_AND { 
    
    self.debug_gaspard("C_AND ");
   return true;
    }if instr &  MASK_C_ANDI ==  MATCH_C_ANDI { 
    
    self.debug_gaspard("C_ANDI ");
   return true;
    }if instr &  MASK_C_BEQZ ==  MATCH_C_BEQZ { 
    
    self.debug_gaspard("C_BEQZ ");
   return true;
    }if instr &  MASK_C_BNEZ ==  MATCH_C_BNEZ { 
    
    self.debug_gaspard("C_BNEZ ");
   return true;
    }if instr &  MASK_C_EBREAK ==  MATCH_C_EBREAK { 
    
    self.debug_gaspard("C_EBREAK ");
   return true;
    }if instr &  MASK_C_FLD ==  MATCH_C_FLD { 
    
    self.debug_gaspard("C_FLD ");
   return true;
    }if instr &  MASK_C_FLDSP ==  MATCH_C_FLDSP { 
    
    self.debug_gaspard("C_FLDSP ");
   return true;
    }if instr &  MASK_C_FLW ==  MATCH_C_FLW { 
    
    self.debug_gaspard("C_FLW ");
   return true;
    }if instr &  MASK_C_FLWSP ==  MATCH_C_FLWSP { 
    
    self.debug_gaspard("C_FLWSP ");
   return true;
    }if instr &  MASK_C_FSD ==  MATCH_C_FSD { 
    
    self.debug_gaspard("C_FSD ");
   return true;
    }if instr &  MASK_C_FSDSP ==  MATCH_C_FSDSP { 
    
    self.debug_gaspard("C_FSDSP ");
   return true;
    }if instr &  MASK_C_FSW ==  MATCH_C_FSW { 
    
    self.debug_gaspard("C_FSW ");
   return true;
    }if instr &  MASK_C_FSWSP ==  MATCH_C_FSWSP { 
    
    self.debug_gaspard("C_FSWSP ");
   return true;
    }if instr &  MASK_C_J ==  MATCH_C_J { 
    
    self.debug_gaspard("C_J ");
   return true;
    }if instr &  MASK_C_JAL ==  MATCH_C_JAL { 
    
    self.debug_gaspard("C_JAL ");
   return true;
    }if instr &  MASK_C_JALR ==  MATCH_C_JALR { 
    
    self.debug_gaspard("C_JALR ");
   return true;
    }if instr &  MASK_C_JR ==  MATCH_C_JR { 
    
    self.debug_gaspard("C_JR ");
   return true;
    }if instr &  MASK_C_LBU ==  MATCH_C_LBU { 
    
    self.debug_gaspard("C_LBU ");
   return true;
    }if instr &  MASK_C_LD ==  MATCH_C_LD { 
    
    self.debug_gaspard("C_LD ");
   return true;
    }if instr &  MASK_C_LDSP ==  MATCH_C_LDSP { 
    
    self.debug_gaspard("C_LDSP ");
   return true;
    }if instr &  MASK_C_LH ==  MATCH_C_LH { 
    
    self.debug_gaspard("C_LH ");
   return true;
    }if instr &  MASK_C_LHU ==  MATCH_C_LHU { 
    
    self.debug_gaspard("C_LHU ");
   return true;
    }if instr &  MASK_C_LI ==  MATCH_C_LI { 
    
    self.debug_gaspard("C_LI ");
   return true;
    }if instr &  MASK_C_LQ ==  MATCH_C_LQ { 
    
    self.debug_gaspard("C_LQ ");
   return true;
    }if instr &  MASK_C_LQSP ==  MATCH_C_LQSP { 
    
    self.debug_gaspard("C_LQSP ");
   return true;
    }if instr &  MASK_C_LUI ==  MATCH_C_LUI { 
    
    self.debug_gaspard("C_LUI ");
   return true;
    }if instr &  MASK_C_LW ==  MATCH_C_LW { 
    
    self.debug_gaspard("C_LW ");
   return true;
    }if instr &  MASK_C_LWSP ==  MATCH_C_LWSP { 
    
    self.debug_gaspard("C_LWSP ");
   return true;
    }if instr &  MASK_C_MOP_1 ==  MATCH_C_MOP_1 { 
    
    self.debug_gaspard("C_MOP_1 ");
   return true;
    }if instr &  MASK_C_MOP_11 ==  MATCH_C_MOP_11 { 
    
    self.debug_gaspard("C_MOP_11 ");
   return true;
    }if instr &  MASK_C_MOP_13 ==  MATCH_C_MOP_13 { 
    
    self.debug_gaspard("C_MOP_13 ");
   return true;
    }if instr &  MASK_C_MOP_15 ==  MATCH_C_MOP_15 { 
    
    self.debug_gaspard("C_MOP_15 ");
   return true;
    }if instr &  MASK_C_MOP_3 ==  MATCH_C_MOP_3 { 
    
    self.debug_gaspard("C_MOP_3 ");
   return true;
    }if instr &  MASK_C_MOP_5 ==  MATCH_C_MOP_5 { 
    
    self.debug_gaspard("C_MOP_5 ");
   return true;
    }if instr &  MASK_C_MOP_7 ==  MATCH_C_MOP_7 { 
    
    self.debug_gaspard("C_MOP_7 ");
   return true;
    }if instr &  MASK_C_MOP_9 ==  MATCH_C_MOP_9 { 
    
    self.debug_gaspard("C_MOP_9 ");
   return true;
    }if instr &  MASK_C_MOP_N ==  MATCH_C_MOP_N { 
    
    self.debug_gaspard("C_MOP_N ");
   return true;
    }if instr &  MASK_C_MUL ==  MATCH_C_MUL { 
    
    self.debug_gaspard("C_MUL ");
   return true;
    }if instr &  MASK_C_MV ==  MATCH_C_MV { 
    
    self.debug_gaspard("C_MV ");
   return true;
    }if instr &  MASK_C_NOP ==  MATCH_C_NOP { 
    
    self.debug_gaspard("C_NOP ");
   return true;
    }if instr &  MASK_C_NOT ==  MATCH_C_NOT { 
    
    self.debug_gaspard("C_NOT ");
   return true;
    }if instr &  MASK_C_NTL_ALL ==  MATCH_C_NTL_ALL { 
    
    self.debug_gaspard("C_NTL_ALL ");
   return true;
    }if instr &  MASK_C_NTL_P1 ==  MATCH_C_NTL_P1 { 
    
    self.debug_gaspard("C_NTL_P1 ");
   return true;
    }if instr &  MASK_C_NTL_PALL ==  MATCH_C_NTL_PALL { 
    
    self.debug_gaspard("C_NTL_PALL ");
   return true;
    }if instr &  MASK_C_NTL_S1 ==  MATCH_C_NTL_S1 { 
    
    self.debug_gaspard("C_NTL_S1 ");
   return true;
    }if instr &  MASK_C_OR ==  MATCH_C_OR { 
    
    self.debug_gaspard("C_OR ");
   return true;
    }if instr &  MASK_C_SB ==  MATCH_C_SB { 
    
    self.debug_gaspard("C_SB ");
   return true;
    }if instr &  MASK_C_SD ==  MATCH_C_SD { 
    
    self.debug_gaspard("C_SD ");
   return true;
    }if instr &  MASK_C_SDSP ==  MATCH_C_SDSP { 
    
    self.debug_gaspard("C_SDSP ");
   return true;
    }if instr &  MASK_C_SEXT_B ==  MATCH_C_SEXT_B { 
    
    self.debug_gaspard("C_SEXT_B ");
   return true;
    }if instr &  MASK_C_SEXT_H ==  MATCH_C_SEXT_H { 
    
    self.debug_gaspard("C_SEXT_H ");
   return true;
    }if instr &  MASK_C_SH ==  MATCH_C_SH { 
    
    self.debug_gaspard("C_SH ");
   return true;
    }if instr &  MASK_C_SLLI ==  MATCH_C_SLLI { 
    
    self.debug_gaspard("C_SLLI ");
   return true;
    }if instr &  MASK_C_SLLI_RV32 ==  MATCH_C_SLLI_RV32 { 
    
    self.debug_gaspard("C_SLLI_RV32 ");
   return true;
    }if instr &  MASK_C_SQ ==  MATCH_C_SQ { 
    
    self.debug_gaspard("C_SQ ");
   return true;
    }if instr &  MASK_C_SQSP ==  MATCH_C_SQSP { 
    
    self.debug_gaspard("C_SQSP ");
   return true;
    }if instr &  MASK_C_SRAI ==  MATCH_C_SRAI { 
    
    self.debug_gaspard("C_SRAI ");
   return true;
    }if instr &  MASK_C_SRAI_RV32 ==  MATCH_C_SRAI_RV32 { 
    
    self.debug_gaspard("C_SRAI_RV32 ");
   return true;
    }if instr &  MASK_C_SRLI ==  MATCH_C_SRLI { 
    
    self.debug_gaspard("C_SRLI ");
   return true;
    }if instr &  MASK_C_SRLI_RV32 ==  MATCH_C_SRLI_RV32 { 
    
    self.debug_gaspard("C_SRLI_RV32 ");
   return true;
    }if instr &  MASK_C_SSPOPCHK_X5 ==  MATCH_C_SSPOPCHK_X5 { 
    
    self.debug_gaspard("C_SSPOPCHK_X5 ");
   return true;
    }if instr &  MASK_C_SSPUSH_X1 ==  MATCH_C_SSPUSH_X1 { 
    
    self.debug_gaspard("C_SSPUSH_X1 ");
   return true;
    }if instr &  MASK_C_SUB ==  MATCH_C_SUB { 
    
    self.debug_gaspard("C_SUB ");
   return true;
    }if instr &  MASK_C_SUBW ==  MATCH_C_SUBW { 
    
    self.debug_gaspard("C_SUBW ");
   return true;
    }if instr &  MASK_C_SW ==  MATCH_C_SW { 
    
    self.debug_gaspard("C_SW ");
   return true;
    }if instr &  MASK_C_SWSP ==  MATCH_C_SWSP { 
    
    self.debug_gaspard("C_SWSP ");
   return true;
    }if instr &  MASK_C_XOR ==  MATCH_C_XOR { 
    
    self.debug_gaspard("C_XOR ");
   return true;
    }if instr &  MASK_C_ZEXT_B ==  MATCH_C_ZEXT_B { 
    
    self.debug_gaspard("C_ZEXT_B ");
   return true;
    }if instr &  MASK_C_ZEXT_H ==  MATCH_C_ZEXT_H { 
    
    self.debug_gaspard("C_ZEXT_H ");
   return true;
    }if instr &  MASK_C_ZEXT_W ==  MATCH_C_ZEXT_W { 
    
    self.debug_gaspard("C_ZEXT_W ");
   return true;
    }if instr &  MASK_CBO_CLEAN ==  MATCH_CBO_CLEAN { 
    
    self.debug_gaspard("CBO_CLEAN ");
   return true;
    }if instr &  MASK_CBO_FLUSH ==  MATCH_CBO_FLUSH { 
    
    self.debug_gaspard("CBO_FLUSH ");
   return true;
    }if instr &  MASK_CBO_INVAL ==  MATCH_CBO_INVAL { 
    
    self.debug_gaspard("CBO_INVAL ");
   return true;
    }if instr &  MASK_CBO_ZERO ==  MATCH_CBO_ZERO { 
    
    self.debug_gaspard("CBO_ZERO ");
   return true;
    }if instr &  MASK_CLMUL ==  MATCH_CLMUL { 
    
    self.debug_gaspard("CLMUL ");
   return true;
    }if instr &  MASK_CLMULH ==  MATCH_CLMULH { 
    
    self.debug_gaspard("CLMULH ");
   return true;
    }if instr &  MASK_CLMULR ==  MATCH_CLMULR { 
    
    self.debug_gaspard("CLMULR ");
   return true;
    }if instr &  MASK_CLROV ==  MATCH_CLROV { 
    
    self.debug_gaspard("CLROV ");
   return true;
    }if instr &  MASK_CLRS16 ==  MATCH_CLRS16 { 
    
    self.debug_gaspard("CLRS16 ");
   return true;
    }if instr &  MASK_CLRS32 ==  MATCH_CLRS32 { 
    
    self.debug_gaspard("CLRS32 ");
   return true;
    }if instr &  MASK_CLRS8 ==  MATCH_CLRS8 { 
    
    self.debug_gaspard("CLRS8 ");
   return true;
    }if instr &  MASK_CLZ ==  MATCH_CLZ { 
    
    self.debug_gaspard("CLZ ");
   return true;
    }if instr &  MASK_CLZ16 ==  MATCH_CLZ16 { 
    
    self.debug_gaspard("CLZ16 ");
   return true;
    }if instr &  MASK_CLZ32 ==  MATCH_CLZ32 { 
    
    self.debug_gaspard("CLZ32 ");
   return true;
    }if instr &  MASK_CLZ8 ==  MATCH_CLZ8 { 
    
    self.debug_gaspard("CLZ8 ");
   return true;
    }if instr &  MASK_CLZW ==  MATCH_CLZW { 
    
    self.debug_gaspard("CLZW ");
   return true;
    }if instr &  MASK_CM_JALT ==  MATCH_CM_JALT { 
    
    self.debug_gaspard("CM_JALT ");
   return true;
    }if instr &  MASK_CM_MVA01S ==  MATCH_CM_MVA01S { 
    
    self.debug_gaspard("CM_MVA01S ");
   return true;
    }if instr &  MASK_CM_MVSA01 ==  MATCH_CM_MVSA01 { 
    
    self.debug_gaspard("CM_MVSA01 ");
   return true;
    }if instr &  MASK_CM_POP ==  MATCH_CM_POP { 
    
    self.debug_gaspard("CM_POP ");
   return true;
    }if instr &  MASK_CM_POPRET ==  MATCH_CM_POPRET { 
    
    self.debug_gaspard("CM_POPRET ");
   return true;
    }if instr &  MASK_CM_POPRETZ ==  MATCH_CM_POPRETZ { 
    
    self.debug_gaspard("CM_POPRETZ ");
   return true;
    }if instr &  MASK_CM_PUSH ==  MATCH_CM_PUSH { 
    
    self.debug_gaspard("CM_PUSH ");
   return true;
    }if instr &  MASK_CMIX ==  MATCH_CMIX { 
    
    self.debug_gaspard("CMIX ");
   return true;
    }if instr &  MASK_CMOV ==  MATCH_CMOV { 
    
    self.debug_gaspard("CMOV ");
   return true;
    }if instr &  MASK_CMPEQ16 ==  MATCH_CMPEQ16 { 
    
    self.debug_gaspard("CMPEQ16 ");
   return true;
    }if instr &  MASK_CMPEQ8 ==  MATCH_CMPEQ8 { 
    
    self.debug_gaspard("CMPEQ8 ");
   return true;
    }if instr &  MASK_CPOP ==  MATCH_CPOP { 
    
    self.debug_gaspard("CPOP ");
   return true;
    }if instr &  MASK_CPOPW ==  MATCH_CPOPW { 
    
    self.debug_gaspard("CPOPW ");
   return true;
    }if instr &  MASK_CRAS16 ==  MATCH_CRAS16 { 
    
    self.debug_gaspard("CRAS16 ");
   return true;
    }if instr &  MASK_CRAS32 ==  MATCH_CRAS32 { 
    
    self.debug_gaspard("CRAS32 ");
   return true;
    }if instr &  MASK_CRC32_B ==  MATCH_CRC32_B { 
    
    self.debug_gaspard("CRC32_B ");
   return true;
    }if instr &  MASK_CRC32_D ==  MATCH_CRC32_D { 
    
    self.debug_gaspard("CRC32_D ");
   return true;
    }if instr &  MASK_CRC32_H ==  MATCH_CRC32_H { 
    
    self.debug_gaspard("CRC32_H ");
   return true;
    }if instr &  MASK_CRC32_W ==  MATCH_CRC32_W { 
    
    self.debug_gaspard("CRC32_W ");
   return true;
    }if instr &  MASK_CRC32C_B ==  MATCH_CRC32C_B { 
    
    self.debug_gaspard("CRC32C_B ");
   return true;
    }if instr &  MASK_CRC32C_D ==  MATCH_CRC32C_D { 
    
    self.debug_gaspard("CRC32C_D ");
   return true;
    }if instr &  MASK_CRC32C_H ==  MATCH_CRC32C_H { 
    
    self.debug_gaspard("CRC32C_H ");
   return true;
    }if instr &  MASK_CRC32C_W ==  MATCH_CRC32C_W { 
    
    self.debug_gaspard("CRC32C_W ");
   return true;
    }if instr &  MASK_CRSA16 ==  MATCH_CRSA16 { 
    
    self.debug_gaspard("CRSA16 ");
   return true;
    }if instr &  MASK_CRSA32 ==  MATCH_CRSA32 { 
    
    self.debug_gaspard("CRSA32 ");
   return true;
    }if instr &  MASK_CSRRC ==  MATCH_CSRRC { 
    
    self.debug_gaspard("CSRRC ");
   return true;
    }if instr &  MASK_CSRRCI ==  MATCH_CSRRCI { 
    
    self.debug_gaspard("CSRRCI ");
   return true;
    }if instr &  MASK_CSRRS ==  MATCH_CSRRS { 
    
    self.debug_gaspard("CSRRS ");
   return true;
    }if instr &  MASK_CSRRSI ==  MATCH_CSRRSI { 
    
    self.debug_gaspard("CSRRSI ");
   return true;
    }if instr &  MASK_CSRRW ==  MATCH_CSRRW { 
    
    self.debug_gaspard("CSRRW ");
   return true;
    }if instr &  MASK_CSRRWI ==  MATCH_CSRRWI { 
    
    self.debug_gaspard("CSRRWI ");
   return true;
    }if instr &  MASK_CTZ ==  MATCH_CTZ { 
    
    self.debug_gaspard("CTZ ");
   return true;
    }if instr &  MASK_CTZW ==  MATCH_CTZW { 
    
    self.debug_gaspard("CTZW ");
   return true;
    }if instr &  MASK_CZERO_EQZ ==  MATCH_CZERO_EQZ { 
    
    self.debug_gaspard("CZERO_EQZ ");
   return true;
    }if instr &  MASK_CZERO_NEZ ==  MATCH_CZERO_NEZ { 
    
    self.debug_gaspard("CZERO_NEZ ");
   return true;
    }if instr &  MASK_DIV ==  MATCH_DIV { 
    
    self.debug_gaspard("DIV ");
   return false;
    }if instr &  MASK_DIVU ==  MATCH_DIVU { 
    
    self.debug_gaspard("DIVU ");
   return false;
    }if instr &  MASK_DIVUW ==  MATCH_DIVUW { 
    
    self.debug_gaspard("DIVUW ");
   return false;
    }if instr &  MASK_DIVW ==  MATCH_DIVW { 
    
    self.debug_gaspard("DIVW ");
   return false;
    }if instr &  MASK_DRET ==  MATCH_DRET { 
    
    self.debug_gaspard("DRET ");
   return false;
    }if instr &  MASK_EBREAK ==  MATCH_EBREAK { 
    
    self.debug_gaspard("EBREAK ");
   return false;
    }if instr &  MASK_ECALL ==  MATCH_ECALL { 
    
    self.debug_gaspard("ECALL ");
   return false;
    }if instr &  MASK_FADD_D ==  MATCH_FADD_D { 
    
    self.debug_gaspard("FADD_D ");
   return false;
    }if instr &  MASK_FADD_H ==  MATCH_FADD_H { 
    
    self.debug_gaspard("FADD_H ");
   return false;
    }if instr &  MASK_FADD_Q ==  MATCH_FADD_Q { 
    
    self.debug_gaspard("FADD_Q ");
   return false;
    }if instr &  MASK_FADD_S ==  MATCH_FADD_S { 
    
    self.debug_gaspard("FADD_S ");
   return false;
    }if instr &  MASK_FCLASS_D ==  MATCH_FCLASS_D { 
    
    self.debug_gaspard("FCLASS_D ");
   return false;
    }if instr &  MASK_FCLASS_H ==  MATCH_FCLASS_H { 
    
    self.debug_gaspard("FCLASS_H ");
   return false;
    }if instr &  MASK_FCLASS_Q ==  MATCH_FCLASS_Q { 
    
    self.debug_gaspard("FCLASS_Q ");
   return false;
    }if instr &  MASK_FCLASS_S ==  MATCH_FCLASS_S { 
    
    self.debug_gaspard("FCLASS_S ");
   return false;
    }if instr &  MASK_FCVT_BF16_S ==  MATCH_FCVT_BF16_S { 
    
    self.debug_gaspard("FCVT_BF16_S ");
   return false;
    }if instr &  MASK_FCVT_D_H ==  MATCH_FCVT_D_H { 
    
    self.debug_gaspard("FCVT_D_H ");
   return false;
    }if instr &  MASK_FCVT_D_L ==  MATCH_FCVT_D_L { 
    
    self.debug_gaspard("FCVT_D_L ");
   return false;
    }if instr &  MASK_FCVT_D_LU ==  MATCH_FCVT_D_LU { 
    
    self.debug_gaspard("FCVT_D_LU ");
   return false;
    }if instr &  MASK_FCVT_D_Q ==  MATCH_FCVT_D_Q { 
    
    self.debug_gaspard("FCVT_D_Q ");
   return false;
    }if instr &  MASK_FCVT_D_S ==  MATCH_FCVT_D_S { 
    
    self.debug_gaspard("FCVT_D_S ");
   return false;
    }if instr &  MASK_FCVT_D_W ==  MATCH_FCVT_D_W { 
    
    self.debug_gaspard("FCVT_D_W ");
   return false;
    }if instr &  MASK_FCVT_D_WU ==  MATCH_FCVT_D_WU { 
    
    self.debug_gaspard("FCVT_D_WU ");
   return false;
    }if instr &  MASK_FCVT_H_D ==  MATCH_FCVT_H_D { 
    
    self.debug_gaspard("FCVT_H_D ");
   return false;
    }if instr &  MASK_FCVT_H_L ==  MATCH_FCVT_H_L { 
    
    self.debug_gaspard("FCVT_H_L ");
   return false;
    }if instr &  MASK_FCVT_H_LU ==  MATCH_FCVT_H_LU { 
    
    self.debug_gaspard("FCVT_H_LU ");
   return false;
    }if instr &  MASK_FCVT_H_Q ==  MATCH_FCVT_H_Q { 
    
    self.debug_gaspard("FCVT_H_Q ");
   return false;
    }if instr &  MASK_FCVT_H_S ==  MATCH_FCVT_H_S { 
    
    self.debug_gaspard("FCVT_H_S ");
   return false;
    }if instr &  MASK_FCVT_H_W ==  MATCH_FCVT_H_W { 
    
    self.debug_gaspard("FCVT_H_W ");
   return false;
    }if instr &  MASK_FCVT_H_WU ==  MATCH_FCVT_H_WU { 
    
    self.debug_gaspard("FCVT_H_WU ");
   return false;
    }if instr &  MASK_FCVT_L_D ==  MATCH_FCVT_L_D { 
    
    self.debug_gaspard("FCVT_L_D ");
   return false;
    }if instr &  MASK_FCVT_L_H ==  MATCH_FCVT_L_H { 
    
    self.debug_gaspard("FCVT_L_H ");
   return false;
    }if instr &  MASK_FCVT_L_Q ==  MATCH_FCVT_L_Q { 
    
    self.debug_gaspard("FCVT_L_Q ");
   return false;
    }if instr &  MASK_FCVT_L_S ==  MATCH_FCVT_L_S { 
    
    self.debug_gaspard("FCVT_L_S ");
   return false;
    }if instr &  MASK_FCVT_LU_D ==  MATCH_FCVT_LU_D { 
    
    self.debug_gaspard("FCVT_LU_D ");
   return false;
    }if instr &  MASK_FCVT_LU_H ==  MATCH_FCVT_LU_H { 
    
    self.debug_gaspard("FCVT_LU_H ");
   return false;
    }if instr &  MASK_FCVT_LU_Q ==  MATCH_FCVT_LU_Q { 
    
    self.debug_gaspard("FCVT_LU_Q ");
   return false;
    }if instr &  MASK_FCVT_LU_S ==  MATCH_FCVT_LU_S { 
    
    self.debug_gaspard("FCVT_LU_S ");
   return false;
    }if instr &  MASK_FCVT_Q_D ==  MATCH_FCVT_Q_D { 
    
    self.debug_gaspard("FCVT_Q_D ");
   return false;
    }if instr &  MASK_FCVT_Q_H ==  MATCH_FCVT_Q_H { 
    
    self.debug_gaspard("FCVT_Q_H ");
   return false;
    }if instr &  MASK_FCVT_Q_L ==  MATCH_FCVT_Q_L { 
    
    self.debug_gaspard("FCVT_Q_L ");
   return false;
    }if instr &  MASK_FCVT_Q_LU ==  MATCH_FCVT_Q_LU { 
    
    self.debug_gaspard("FCVT_Q_LU ");
   return false;
    }if instr &  MASK_FCVT_Q_S ==  MATCH_FCVT_Q_S { 
    
    self.debug_gaspard("FCVT_Q_S ");
   return false;
    }if instr &  MASK_FCVT_Q_W ==  MATCH_FCVT_Q_W { 
    
    self.debug_gaspard("FCVT_Q_W ");
   return false;
    }if instr &  MASK_FCVT_Q_WU ==  MATCH_FCVT_Q_WU { 
    
    self.debug_gaspard("FCVT_Q_WU ");
   return false;
    }if instr &  MASK_FCVT_S_BF16 ==  MATCH_FCVT_S_BF16 { 
    
    self.debug_gaspard("FCVT_S_BF16 ");
   return false;
    }if instr &  MASK_FCVT_S_D ==  MATCH_FCVT_S_D { 
    
    self.debug_gaspard("FCVT_S_D ");
   return false;
    }if instr &  MASK_FCVT_S_H ==  MATCH_FCVT_S_H { 
    
    self.debug_gaspard("FCVT_S_H ");
   return false;
    }if instr &  MASK_FCVT_S_L ==  MATCH_FCVT_S_L { 
    
    self.debug_gaspard("FCVT_S_L ");
   return false;
    }if instr &  MASK_FCVT_S_LU ==  MATCH_FCVT_S_LU { 
    
    self.debug_gaspard("FCVT_S_LU ");
   return false;
    }if instr &  MASK_FCVT_S_Q ==  MATCH_FCVT_S_Q { 
    
    self.debug_gaspard("FCVT_S_Q ");
   return false;
    }if instr &  MASK_FCVT_S_W ==  MATCH_FCVT_S_W { 
    
    self.debug_gaspard("FCVT_S_W ");
   return false;
    }if instr &  MASK_FCVT_S_WU ==  MATCH_FCVT_S_WU { 
    
    self.debug_gaspard("FCVT_S_WU ");
   return false;
    }if instr &  MASK_FCVT_W_D ==  MATCH_FCVT_W_D { 
    
    self.debug_gaspard("FCVT_W_D ");
   return false;
    }if instr &  MASK_FCVT_W_H ==  MATCH_FCVT_W_H { 
    
    self.debug_gaspard("FCVT_W_H ");
   return false;
    }if instr &  MASK_FCVT_W_Q ==  MATCH_FCVT_W_Q { 
    
    self.debug_gaspard("FCVT_W_Q ");
   return false;
    }if instr &  MASK_FCVT_W_S ==  MATCH_FCVT_W_S { 
    
    self.debug_gaspard("FCVT_W_S ");
   return false;
    }if instr &  MASK_FCVT_WU_D ==  MATCH_FCVT_WU_D { 
    
    self.debug_gaspard("FCVT_WU_D ");
   return false;
    }if instr &  MASK_FCVT_WU_H ==  MATCH_FCVT_WU_H { 
    
    self.debug_gaspard("FCVT_WU_H ");
   return false;
    }if instr &  MASK_FCVT_WU_Q ==  MATCH_FCVT_WU_Q { 
    
    self.debug_gaspard("FCVT_WU_Q ");
   return false;
    }if instr &  MASK_FCVT_WU_S ==  MATCH_FCVT_WU_S { 
    
    self.debug_gaspard("FCVT_WU_S ");
   return false;
    }if instr &  MASK_FCVTMOD_W_D ==  MATCH_FCVTMOD_W_D { 
    
    self.debug_gaspard("FCVTMOD_W_D ");
   return false;
    }if instr &  MASK_FDIV_D ==  MATCH_FDIV_D { 
    
    self.debug_gaspard("FDIV_D ");
   return false;
    }if instr &  MASK_FDIV_H ==  MATCH_FDIV_H { 
    
    self.debug_gaspard("FDIV_H ");
   return false;
    }if instr &  MASK_FDIV_Q ==  MATCH_FDIV_Q { 
    
    self.debug_gaspard("FDIV_Q ");
   return false;
    }if instr &  MASK_FDIV_S ==  MATCH_FDIV_S { 
    
    self.debug_gaspard("FDIV_S ");
   return false;
    }if instr &  MASK_FENCE ==  MATCH_FENCE { 
    
    self.debug_gaspard("FENCE ");
   return false;
    }if instr &  MASK_FENCE_I ==  MATCH_FENCE_I { 
    
    self.debug_gaspard("FENCE_I ");
   return false;
    }if instr &  MASK_FENCE_TSO ==  MATCH_FENCE_TSO { 
    
    self.debug_gaspard("FENCE_TSO ");
   return false;
    }if instr &  MASK_FEQ_D ==  MATCH_FEQ_D { 
    
    self.debug_gaspard("FEQ_D ");
   return false;
    }if instr &  MASK_FEQ_H ==  MATCH_FEQ_H { 
    
    self.debug_gaspard("FEQ_H ");
   return false;
    }if instr &  MASK_FEQ_Q ==  MATCH_FEQ_Q { 
    
    self.debug_gaspard("FEQ_Q ");
   return false;
    }if instr &  MASK_FEQ_S ==  MATCH_FEQ_S { 
    
    self.debug_gaspard("FEQ_S ");
   return false;
    }if instr &  MASK_FLD ==  MATCH_FLD { 
    
    self.debug_gaspard("FLD ");
   return false;
    }if instr &  MASK_FLE_D ==  MATCH_FLE_D { 
    
    self.debug_gaspard("FLE_D ");
   return false;
    }if instr &  MASK_FLE_H ==  MATCH_FLE_H { 
    
    self.debug_gaspard("FLE_H ");
   return false;
    }if instr &  MASK_FLE_Q ==  MATCH_FLE_Q { 
    
    self.debug_gaspard("FLE_Q ");
   return false;
    }if instr &  MASK_FLE_S ==  MATCH_FLE_S { 
    
    self.debug_gaspard("FLE_S ");
   return false;
    }if instr &  MASK_FLEQ_D ==  MATCH_FLEQ_D { 
    
    self.debug_gaspard("FLEQ_D ");
   return false;
    }if instr &  MASK_FLEQ_H ==  MATCH_FLEQ_H { 
    
    self.debug_gaspard("FLEQ_H ");
   return false;
    }if instr &  MASK_FLEQ_Q ==  MATCH_FLEQ_Q { 
    
    self.debug_gaspard("FLEQ_Q ");
   return false;
    }if instr &  MASK_FLEQ_S ==  MATCH_FLEQ_S { 
    
    self.debug_gaspard("FLEQ_S ");
   return false;
    }if instr &  MASK_FLH ==  MATCH_FLH { 
    
    self.debug_gaspard("FLH ");
   return false;
    }if instr &  MASK_FLI_D ==  MATCH_FLI_D { 
    
    self.debug_gaspard("FLI_D ");
   return false;
    }if instr &  MASK_FLI_H ==  MATCH_FLI_H { 
    
    self.debug_gaspard("FLI_H ");
   return false;
    }if instr &  MASK_FLI_Q ==  MATCH_FLI_Q { 
    
    self.debug_gaspard("FLI_Q ");
   return false;
    }if instr &  MASK_FLI_S ==  MATCH_FLI_S { 
    
    self.debug_gaspard("FLI_S ");
   return false;
    }if instr &  MASK_FLQ ==  MATCH_FLQ { 
    
    self.debug_gaspard("FLQ ");
   return false;
    }if instr &  MASK_FLT_D ==  MATCH_FLT_D { 
    
    self.debug_gaspard("FLT_D ");
   return false;
    }if instr &  MASK_FLT_H ==  MATCH_FLT_H { 
    
    self.debug_gaspard("FLT_H ");
   return false;
    }if instr &  MASK_FLT_Q ==  MATCH_FLT_Q { 
    
    self.debug_gaspard("FLT_Q ");
   return false;
    }if instr &  MASK_FLT_S ==  MATCH_FLT_S { 
    
    self.debug_gaspard("FLT_S ");
   return false;
    }if instr &  MASK_FLTQ_D ==  MATCH_FLTQ_D { 
    
    self.debug_gaspard("FLTQ_D ");
   return false;
    }if instr &  MASK_FLTQ_H ==  MATCH_FLTQ_H { 
    
    self.debug_gaspard("FLTQ_H ");
   return false;
    }if instr &  MASK_FLTQ_Q ==  MATCH_FLTQ_Q { 
    
    self.debug_gaspard("FLTQ_Q ");
   return false;
    }if instr &  MASK_FLTQ_S ==  MATCH_FLTQ_S { 
    
    self.debug_gaspard("FLTQ_S ");
   return false;
    }if instr &  MASK_FLW ==  MATCH_FLW { 
    
    self.debug_gaspard("FLW ");
   return false;
    }if instr &  MASK_FMADD_D ==  MATCH_FMADD_D { 
    
    self.debug_gaspard("FMADD_D ");
   return false;
    }if instr &  MASK_FMADD_H ==  MATCH_FMADD_H { 
    
    self.debug_gaspard("FMADD_H ");
   return false;
    }if instr &  MASK_FMADD_Q ==  MATCH_FMADD_Q { 
    
    self.debug_gaspard("FMADD_Q ");
   return false;
    }if instr &  MASK_FMADD_S ==  MATCH_FMADD_S { 
    
    self.debug_gaspard("FMADD_S ");
   return false;
    }if instr &  MASK_FMAX_D ==  MATCH_FMAX_D { 
    
    self.debug_gaspard("FMAX_D ");
   return false;
    }if instr &  MASK_FMAX_H ==  MATCH_FMAX_H { 
    
    self.debug_gaspard("FMAX_H ");
   return false;
    }if instr &  MASK_FMAX_Q ==  MATCH_FMAX_Q { 
    
    self.debug_gaspard("FMAX_Q ");
   return false;
    }if instr &  MASK_FMAX_S ==  MATCH_FMAX_S { 
    
    self.debug_gaspard("FMAX_S ");
   return false;
    }if instr &  MASK_FMAXM_D ==  MATCH_FMAXM_D { 
    
    self.debug_gaspard("FMAXM_D ");
   return false;
    }if instr &  MASK_FMAXM_H ==  MATCH_FMAXM_H { 
    
    self.debug_gaspard("FMAXM_H ");
   return false;
    }if instr &  MASK_FMAXM_Q ==  MATCH_FMAXM_Q { 
    
    self.debug_gaspard("FMAXM_Q ");
   return false;
    }if instr &  MASK_FMAXM_S ==  MATCH_FMAXM_S { 
    
    self.debug_gaspard("FMAXM_S ");
   return false;
    }if instr &  MASK_FMIN_D ==  MATCH_FMIN_D { 
    
    self.debug_gaspard("FMIN_D ");
   return false;
    }if instr &  MASK_FMIN_H ==  MATCH_FMIN_H { 
    
    self.debug_gaspard("FMIN_H ");
   return false;
    }if instr &  MASK_FMIN_Q ==  MATCH_FMIN_Q { 
    
    self.debug_gaspard("FMIN_Q ");
   return false;
    }if instr &  MASK_FMIN_S ==  MATCH_FMIN_S { 
    
    self.debug_gaspard("FMIN_S ");
   return false;
    }if instr &  MASK_FMINM_D ==  MATCH_FMINM_D { 
    
    self.debug_gaspard("FMINM_D ");
   return false;
    }if instr &  MASK_FMINM_H ==  MATCH_FMINM_H { 
    
    self.debug_gaspard("FMINM_H ");
   return false;
    }if instr &  MASK_FMINM_Q ==  MATCH_FMINM_Q { 
    
    self.debug_gaspard("FMINM_Q ");
   return false;
    }if instr &  MASK_FMINM_S ==  MATCH_FMINM_S { 
    
    self.debug_gaspard("FMINM_S ");
   return false;
    }if instr &  MASK_FMSUB_D ==  MATCH_FMSUB_D { 
    
    self.debug_gaspard("FMSUB_D ");
   return false;
    }if instr &  MASK_FMSUB_H ==  MATCH_FMSUB_H { 
    
    self.debug_gaspard("FMSUB_H ");
   return false;
    }if instr &  MASK_FMSUB_Q ==  MATCH_FMSUB_Q { 
    
    self.debug_gaspard("FMSUB_Q ");
   return false;
    }if instr &  MASK_FMSUB_S ==  MATCH_FMSUB_S { 
    
    self.debug_gaspard("FMSUB_S ");
   return false;
    }if instr &  MASK_FMUL_D ==  MATCH_FMUL_D { 
    
    self.debug_gaspard("FMUL_D ");
   return false;
    }if instr &  MASK_FMUL_H ==  MATCH_FMUL_H { 
    
    self.debug_gaspard("FMUL_H ");
   return false;
    }if instr &  MASK_FMUL_Q ==  MATCH_FMUL_Q { 
    
    self.debug_gaspard("FMUL_Q ");
   return false;
    }if instr &  MASK_FMUL_S ==  MATCH_FMUL_S { 
    
    self.debug_gaspard("FMUL_S ");
   return false;
    }if instr &  MASK_FMV_D_X ==  MATCH_FMV_D_X { 
    
    self.debug_gaspard("FMV_D_X ");
   return false;
    }if instr &  MASK_FMV_H_X ==  MATCH_FMV_H_X { 
    
    self.debug_gaspard("FMV_H_X ");
   return false;
    }if instr &  MASK_FMV_S_X ==  MATCH_FMV_S_X { 
    
    self.debug_gaspard("FMV_S_X ");
   return false;
    }if instr &  MASK_FMV_W_X ==  MATCH_FMV_W_X { 
    
    self.debug_gaspard("FMV_W_X ");
   return false;
    }if instr &  MASK_FMV_X_D ==  MATCH_FMV_X_D { 
    
    self.debug_gaspard("FMV_X_D ");
   return false;
    }if instr &  MASK_FMV_X_H ==  MATCH_FMV_X_H { 
    
    self.debug_gaspard("FMV_X_H ");
   return false;
    }if instr &  MASK_FMV_X_S ==  MATCH_FMV_X_S { 
    
    self.debug_gaspard("FMV_X_S ");
   return false;
    }if instr &  MASK_FMV_X_W ==  MATCH_FMV_X_W { 
    
    self.debug_gaspard("FMV_X_W ");
   return false;
    }if instr &  MASK_FMVH_X_D ==  MATCH_FMVH_X_D { 
    
    self.debug_gaspard("FMVH_X_D ");
   return false;
    }if instr &  MASK_FMVH_X_Q ==  MATCH_FMVH_X_Q { 
    
    self.debug_gaspard("FMVH_X_Q ");
   return false;
    }if instr &  MASK_FMVP_D_X ==  MATCH_FMVP_D_X { 
    
    self.debug_gaspard("FMVP_D_X ");
   return false;
    }if instr &  MASK_FMVP_Q_X ==  MATCH_FMVP_Q_X { 
    
    self.debug_gaspard("FMVP_Q_X ");
   return false;
    }if instr &  MASK_FNMADD_D ==  MATCH_FNMADD_D { 
    
    self.debug_gaspard("FNMADD_D ");
   return false;
    }if instr &  MASK_FNMADD_H ==  MATCH_FNMADD_H { 
    
    self.debug_gaspard("FNMADD_H ");
   return false;
    }if instr &  MASK_FNMADD_Q ==  MATCH_FNMADD_Q { 
    
    self.debug_gaspard("FNMADD_Q ");
   return false;
    }if instr &  MASK_FNMADD_S ==  MATCH_FNMADD_S { 
    
    self.debug_gaspard("FNMADD_S ");
   return false;
    }if instr &  MASK_FNMSUB_D ==  MATCH_FNMSUB_D { 
    
    self.debug_gaspard("FNMSUB_D ");
   return false;
    }if instr &  MASK_FNMSUB_H ==  MATCH_FNMSUB_H { 
    
    self.debug_gaspard("FNMSUB_H ");
   return false;
    }if instr &  MASK_FNMSUB_Q ==  MATCH_FNMSUB_Q { 
    
    self.debug_gaspard("FNMSUB_Q ");
   return false;
    }if instr &  MASK_FNMSUB_S ==  MATCH_FNMSUB_S { 
    
    self.debug_gaspard("FNMSUB_S ");
   return false;
    }if instr &  MASK_FRCSR ==  MATCH_FRCSR { 
    
    self.debug_gaspard("FRCSR ");
   return false;
    }if instr &  MASK_FRFLAGS ==  MATCH_FRFLAGS { 
    
    self.debug_gaspard("FRFLAGS ");
   return false;
    }if instr &  MASK_FROUND_D ==  MATCH_FROUND_D { 
    
    self.debug_gaspard("FROUND_D ");
   return false;
    }if instr &  MASK_FROUND_H ==  MATCH_FROUND_H { 
    
    self.debug_gaspard("FROUND_H ");
   return false;
    }if instr &  MASK_FROUND_Q ==  MATCH_FROUND_Q { 
    
    self.debug_gaspard("FROUND_Q ");
   return false;
    }if instr &  MASK_FROUND_S ==  MATCH_FROUND_S { 
    
    self.debug_gaspard("FROUND_S ");
   return false;
    }if instr &  MASK_FROUNDNX_D ==  MATCH_FROUNDNX_D { 
    
    self.debug_gaspard("FROUNDNX_D ");
   return false;
    }if instr &  MASK_FROUNDNX_H ==  MATCH_FROUNDNX_H { 
    
    self.debug_gaspard("FROUNDNX_H ");
   return false;
    }if instr &  MASK_FROUNDNX_Q ==  MATCH_FROUNDNX_Q { 
    
    self.debug_gaspard("FROUNDNX_Q ");
   return false;
    }if instr &  MASK_FROUNDNX_S ==  MATCH_FROUNDNX_S { 
    
    self.debug_gaspard("FROUNDNX_S ");
   return false;
    }if instr &  MASK_FRRM ==  MATCH_FRRM { 
    
    self.debug_gaspard("FRRM ");
   return false;
    }if instr &  MASK_FSCSR ==  MATCH_FSCSR { 
    
    self.debug_gaspard("FSCSR ");
   return false;
    }if instr &  MASK_FSD ==  MATCH_FSD { 
    
    self.debug_gaspard("FSD ");
   return false;
    }if instr &  MASK_FSFLAGS ==  MATCH_FSFLAGS { 
    
    self.debug_gaspard("FSFLAGS ");
   return false;
    }if instr &  MASK_FSFLAGSI ==  MATCH_FSFLAGSI { 
    
    self.debug_gaspard("FSFLAGSI ");
   return false;
    }if instr &  MASK_FSGNJ_D ==  MATCH_FSGNJ_D { 
    
    self.debug_gaspard("FSGNJ_D ");
   return false;
    }if instr &  MASK_FSGNJ_H ==  MATCH_FSGNJ_H { 
    
    self.debug_gaspard("FSGNJ_H ");
   return false;
    }if instr &  MASK_FSGNJ_Q ==  MATCH_FSGNJ_Q { 
    
    self.debug_gaspard("FSGNJ_Q ");
   return false;
    }if instr &  MASK_FSGNJ_S ==  MATCH_FSGNJ_S { 
    
    self.debug_gaspard("FSGNJ_S ");
   return false;
    }if instr &  MASK_FSGNJN_D ==  MATCH_FSGNJN_D { 
    
    self.debug_gaspard("FSGNJN_D ");
   return false;
    }if instr &  MASK_FSGNJN_H ==  MATCH_FSGNJN_H { 
    
    self.debug_gaspard("FSGNJN_H ");
   return false;
    }if instr &  MASK_FSGNJN_Q ==  MATCH_FSGNJN_Q { 
    
    self.debug_gaspard("FSGNJN_Q ");
   return false;
    }if instr &  MASK_FSGNJN_S ==  MATCH_FSGNJN_S { 
    
    self.debug_gaspard("FSGNJN_S ");
   return false;
    }if instr &  MASK_FSGNJX_D ==  MATCH_FSGNJX_D { 
    
    self.debug_gaspard("FSGNJX_D ");
   return false;
    }if instr &  MASK_FSGNJX_H ==  MATCH_FSGNJX_H { 
    
    self.debug_gaspard("FSGNJX_H ");
   return false;
    }if instr &  MASK_FSGNJX_Q ==  MATCH_FSGNJX_Q { 
    
    self.debug_gaspard("FSGNJX_Q ");
   return false;
    }if instr &  MASK_FSGNJX_S ==  MATCH_FSGNJX_S { 
    
    self.debug_gaspard("FSGNJX_S ");
   return false;
    }if instr &  MASK_FSH ==  MATCH_FSH { 
    
    self.debug_gaspard("FSH ");
   return false;
    }if instr &  MASK_FSL ==  MATCH_FSL { 
    
    self.debug_gaspard("FSL ");
   return false;
    }if instr &  MASK_FSLW ==  MATCH_FSLW { 
    
    self.debug_gaspard("FSLW ");
   return false;
    }if instr &  MASK_FSQ ==  MATCH_FSQ { 
    
    self.debug_gaspard("FSQ ");
   return false;
    }if instr &  MASK_FSQRT_D ==  MATCH_FSQRT_D { 
    
    self.debug_gaspard("FSQRT_D ");
   return false;
    }if instr &  MASK_FSQRT_H ==  MATCH_FSQRT_H { 
    
    self.debug_gaspard("FSQRT_H ");
   return false;
    }if instr &  MASK_FSQRT_Q ==  MATCH_FSQRT_Q { 
    
    self.debug_gaspard("FSQRT_Q ");
   return false;
    }if instr &  MASK_FSQRT_S ==  MATCH_FSQRT_S { 
    
    self.debug_gaspard("FSQRT_S ");
   return false;
    }if instr &  MASK_FSR ==  MATCH_FSR { 
    
    self.debug_gaspard("FSR ");
   return false;
    }if instr &  MASK_FSRI ==  MATCH_FSRI { 
    
    self.debug_gaspard("FSRI ");
   return false;
    }if instr &  MASK_FSRIW ==  MATCH_FSRIW { 
    
    self.debug_gaspard("FSRIW ");
   return false;
    }if instr &  MASK_FSRM ==  MATCH_FSRM { 
    
    self.debug_gaspard("FSRM ");
   return false;
    }if instr &  MASK_FSRMI ==  MATCH_FSRMI { 
    
    self.debug_gaspard("FSRMI ");
   return false;
    }if instr &  MASK_FSRW ==  MATCH_FSRW { 
    
    self.debug_gaspard("FSRW ");
   return false;
    }if instr &  MASK_FSUB_D ==  MATCH_FSUB_D { 
    
    self.debug_gaspard("FSUB_D ");
   return false;
    }if instr &  MASK_FSUB_H ==  MATCH_FSUB_H { 
    
    self.debug_gaspard("FSUB_H ");
   return false;
    }if instr &  MASK_FSUB_Q ==  MATCH_FSUB_Q { 
    
    self.debug_gaspard("FSUB_Q ");
   return false;
    }if instr &  MASK_FSUB_S ==  MATCH_FSUB_S { 
    
    self.debug_gaspard("FSUB_S ");
   return false;
    }if instr &  MASK_FSW ==  MATCH_FSW { 
    
    self.debug_gaspard("FSW ");
   return false;
    }if instr &  MASK_GORC ==  MATCH_GORC { 
    
    self.debug_gaspard("GORC ");
   return false;
    }if instr &  MASK_GORCI ==  MATCH_GORCI { 
    
    self.debug_gaspard("GORCI ");
   return false;
    }if instr &  MASK_GORCIW ==  MATCH_GORCIW { 
    
    self.debug_gaspard("GORCIW ");
   return false;
    }if instr &  MASK_GORCW ==  MATCH_GORCW { 
    
    self.debug_gaspard("GORCW ");
   return false;
    }if instr &  MASK_GREV ==  MATCH_GREV { 
    
    self.debug_gaspard("GREV ");
   return false;
    }if instr &  MASK_GREVI ==  MATCH_GREVI { 
    
    self.debug_gaspard("GREVI ");
   return false;
    }if instr &  MASK_GREVIW ==  MATCH_GREVIW { 
    
    self.debug_gaspard("GREVIW ");
   return false;
    }if instr &  MASK_GREVW ==  MATCH_GREVW { 
    
    self.debug_gaspard("GREVW ");
   return false;
    }if instr &  MASK_HFENCE_GVMA ==  MATCH_HFENCE_GVMA { 
    
    self.debug_gaspard("HFENCE_GVMA ");
   return false;
    }if instr &  MASK_HFENCE_VVMA ==  MATCH_HFENCE_VVMA { 
    
    self.debug_gaspard("HFENCE_VVMA ");
   return false;
    }if instr &  MASK_HINVAL_GVMA ==  MATCH_HINVAL_GVMA { 
    
    self.debug_gaspard("HINVAL_GVMA ");
   return false;
    }if instr &  MASK_HINVAL_VVMA ==  MATCH_HINVAL_VVMA { 
    
    self.debug_gaspard("HINVAL_VVMA ");
   return false;
    }if instr &  MASK_HLV_B ==  MATCH_HLV_B { 
    
    self.debug_gaspard("HLV_B ");
   return false;
    }if instr &  MASK_HLV_BU ==  MATCH_HLV_BU { 
    
    self.debug_gaspard("HLV_BU ");
   return false;
    }if instr &  MASK_HLV_D ==  MATCH_HLV_D { 
    
    self.debug_gaspard("HLV_D ");
   return false;
    }if instr &  MASK_HLV_H ==  MATCH_HLV_H { 
    
    self.debug_gaspard("HLV_H ");
   return false;
    }if instr &  MASK_HLV_HU ==  MATCH_HLV_HU { 
    
    self.debug_gaspard("HLV_HU ");
   return false;
    }if instr &  MASK_HLV_W ==  MATCH_HLV_W { 
    
    self.debug_gaspard("HLV_W ");
   return false;
    }if instr &  MASK_HLV_WU ==  MATCH_HLV_WU { 
    
    self.debug_gaspard("HLV_WU ");
   return false;
    }if instr &  MASK_HLVX_HU ==  MATCH_HLVX_HU { 
    
    self.debug_gaspard("HLVX_HU ");
   return false;
    }if instr &  MASK_HLVX_WU ==  MATCH_HLVX_WU { 
    
    self.debug_gaspard("HLVX_WU ");
   return false;
    }if instr &  MASK_HSV_B ==  MATCH_HSV_B { 
    
    self.debug_gaspard("HSV_B ");
   return false;
    }if instr &  MASK_HSV_D ==  MATCH_HSV_D { 
    
    self.debug_gaspard("HSV_D ");
   return false;
    }if instr &  MASK_HSV_H ==  MATCH_HSV_H { 
    
    self.debug_gaspard("HSV_H ");
   return false;
    }if instr &  MASK_HSV_W ==  MATCH_HSV_W { 
    
    self.debug_gaspard("HSV_W ");
   return false;
    }if instr &  MASK_INSB ==  MATCH_INSB { 
    
    self.debug_gaspard("INSB ");
   return false;
    }if instr &  MASK_JAL ==  MATCH_JAL { 
    
    self.debug_gaspard("JAL instr : ");
        println!("instr jal {:x}",instr);

	
    let first: u32 = instr >> 12 & 0b11111111; //imm[19;12] 
    let second : u32 = instr  >> 20 & 0b1; // imm[11]
    let third : u32 = instr  >> 21 & 0b1111111111; // imm[10:1]
    let fourth: u32 = instr >> 31 & 0b1; // imm[20]
    let rd : u32 = instr >> 7 & 0b1111;

    let imm = instr >> 12;
    let imm_final : i32 =(   third << 1 |  second << 11 |first << 12 | fourth  << 20).try_into().unwrap();
    println!("imm final {} /b : {:b} num of bits : {} num of zeros: {} ",imm,imm,imm_final.count_ones(),imm_final.count_zeros());

    let bourrage = imm_final.count_zeros();
    let imm_signd_final : i32 = (imm_final  | (0xffffffu32 as i32) << 32-imm_final.count_ones()) as i32;

    let tmp: i32 = self.sign_extend(imm.try_into().unwrap(),imm_final.count_zeros());

    println!("jal impression imm {:x} imm_final  {:x}  imm_final  /d {}  imm_final  /b {:b} adress_debut_fonction {:x} ",imm_signd_final, imm_signd_final,imm_signd_final ,imm_signd_final,self.adress_debut_fonction);

    self.regs[rd as usize] = self.regs[32];
    self.adress_debut_fonction = self.regs[32]; // très important     

    self.regs[32] = (self.adress_debut_fonction.wrapping_add(imm_signd_final as u64)as i64) .abs() as u64 -4;

  
    //self.regs[32] = (self.adress_debut_fonction as u64) + (imm_signd_final as u64); 

    
    
    println!("final self.Regs[32] {:x} en binaire : {:b}", self.regs[32],self.regs[32]);
    
   return false;
    }if instr &  MASK_JALR ==  MATCH_JALR { 
    
    self.debug_gaspard("JALR ");


    let imm : u32 = instr >> 20;
    let signed_imm : i32 = (imm |  0b111111111111 << 20) as i32;
        let rs1 : u32 = instr >> 15 & 0b1111;



        let rd : u32 = instr >> 7 & 0b1111;
    

        if rs1 == 1 && rd == 0 && imm ==0{
// retour à le sous routine
            self.regs[32] = self.adress_debut_fonction + 4 ;// incrementer sinon on revient sur le même branchement c'est très con 

        } else {

        self.regs[rd as usize ] = self.regs[32] + imm as u64 + self.regs[rs1 as usize] as u64; // resultat bizarre .... 


        self.regs[32] =  self.regs[rd as usize ] ;
    }
        println!("jalr adress : {:x}", self.regs[rd as usize ]);

   return false;
    }if instr &  MASK_KABS16 ==  MATCH_KABS16 { 
    
    self.debug_gaspard("KABS16 ");
   return false;
    }if instr &  MASK_KABS32 ==  MATCH_KABS32 { 
    
    self.debug_gaspard("KABS32 ");
   return false;
    }if instr &  MASK_KABS8 ==  MATCH_KABS8 { 
    
    self.debug_gaspard("KABS8 ");
   return false;
    }if instr &  MASK_KABSW ==  MATCH_KABSW { 
    
    self.debug_gaspard("KABSW ");
   return false;
    }if instr &  MASK_KADD16 ==  MATCH_KADD16 { 
    
    self.debug_gaspard("KADD16 ");
   return false;
    }if instr &  MASK_KADD32 ==  MATCH_KADD32 { 
    
    self.debug_gaspard("KADD32 ");
   return false;
    }if instr &  MASK_KADD64 ==  MATCH_KADD64 { 
    
    self.debug_gaspard("KADD64 ");
   return false;
    }if instr &  MASK_KADD8 ==  MATCH_KADD8 { 
    
    self.debug_gaspard("KADD8 ");
   return false;
    }if instr &  MASK_KADDH ==  MATCH_KADDH { 
    
    self.debug_gaspard("KADDH ");
   return false;
    }if instr &  MASK_KADDW ==  MATCH_KADDW { 
    
    self.debug_gaspard("KADDW ");
   return false;
    }if instr &  MASK_KCRAS16 ==  MATCH_KCRAS16 { 
    
    self.debug_gaspard("KCRAS16 ");
   return false;
    }if instr &  MASK_KCRAS32 ==  MATCH_KCRAS32 { 
    
    self.debug_gaspard("KCRAS32 ");
   return false;
    }if instr &  MASK_KCRSA16 ==  MATCH_KCRSA16 { 
    
    self.debug_gaspard("KCRSA16 ");
   return false;
    }if instr &  MASK_KCRSA32 ==  MATCH_KCRSA32 { 
    
    self.debug_gaspard("KCRSA32 ");
   return false;
    }if instr &  MASK_KDMABB ==  MATCH_KDMABB { 
    
    self.debug_gaspard("KDMABB ");
   return false;
    }if instr &  MASK_KDMABB16 ==  MATCH_KDMABB16 { 
    
    self.debug_gaspard("KDMABB16 ");
   return false;
    }if instr &  MASK_KDMABT ==  MATCH_KDMABT { 
    
    self.debug_gaspard("KDMABT ");
   return false;
    }if instr &  MASK_KDMABT16 ==  MATCH_KDMABT16 { 
    
    self.debug_gaspard("KDMABT16 ");
   return false;
    }if instr &  MASK_KDMATT ==  MATCH_KDMATT { 
    
    self.debug_gaspard("KDMATT ");
   return false;
    }if instr &  MASK_KDMATT16 ==  MATCH_KDMATT16 { 
    
    self.debug_gaspard("KDMATT16 ");
   return false;
    }if instr &  MASK_KDMBB ==  MATCH_KDMBB { 
    
    self.debug_gaspard("KDMBB ");
   return false;
    }if instr &  MASK_KDMBB16 ==  MATCH_KDMBB16 { 
    
    self.debug_gaspard("KDMBB16 ");
   return false;
    }if instr &  MASK_KDMBT ==  MATCH_KDMBT { 
    
    self.debug_gaspard("KDMBT ");
   return false;
    }if instr &  MASK_KDMBT16 ==  MATCH_KDMBT16 { 
    
    self.debug_gaspard("KDMBT16 ");
   return false;
    }if instr &  MASK_KDMTT ==  MATCH_KDMTT { 
    
    self.debug_gaspard("KDMTT ");
   return false;
    }if instr &  MASK_KDMTT16 ==  MATCH_KDMTT16 { 
    
    self.debug_gaspard("KDMTT16 ");
   return false;
    }if instr &  MASK_KHM16 ==  MATCH_KHM16 { 
    
    self.debug_gaspard("KHM16 ");
   return false;
    }if instr &  MASK_KHM8 ==  MATCH_KHM8 { 
    
    self.debug_gaspard("KHM8 ");
   return false;
    }if instr &  MASK_KHMBB ==  MATCH_KHMBB { 
    
    self.debug_gaspard("KHMBB ");
   return false;
    }if instr &  MASK_KHMBB16 ==  MATCH_KHMBB16 { 
    
    self.debug_gaspard("KHMBB16 ");
   return false;
    }if instr &  MASK_KHMBT ==  MATCH_KHMBT { 
    
    self.debug_gaspard("KHMBT ");
   return false;
    }if instr &  MASK_KHMBT16 ==  MATCH_KHMBT16 { 
    
    self.debug_gaspard("KHMBT16 ");
   return false;
    }if instr &  MASK_KHMTT ==  MATCH_KHMTT { 
    
    self.debug_gaspard("KHMTT ");
   return false;
    }if instr &  MASK_KHMTT16 ==  MATCH_KHMTT16 { 
    
    self.debug_gaspard("KHMTT16 ");
   return false;
    }if instr &  MASK_KHMX16 ==  MATCH_KHMX16 { 
    
    self.debug_gaspard("KHMX16 ");
   return false;
    }if instr &  MASK_KHMX8 ==  MATCH_KHMX8 { 
    
    self.debug_gaspard("KHMX8 ");
   return false;
    }if instr &  MASK_KMABB ==  MATCH_KMABB { 
    
    self.debug_gaspard("KMABB ");
   return false;
    }if instr &  MASK_KMABB32 ==  MATCH_KMABB32 { 
    
    self.debug_gaspard("KMABB32 ");
   return false;
    }if instr &  MASK_KMABT ==  MATCH_KMABT { 
    
    self.debug_gaspard("KMABT ");
   return false;
    }if instr &  MASK_KMABT32 ==  MATCH_KMABT32 { 
    
    self.debug_gaspard("KMABT32 ");
   return false;
    }if instr &  MASK_KMADA ==  MATCH_KMADA { 
    
    self.debug_gaspard("KMADA ");
   return false;
    }if instr &  MASK_KMADRS ==  MATCH_KMADRS { 
    
    self.debug_gaspard("KMADRS ");
   return false;
    }if instr &  MASK_KMADRS32 ==  MATCH_KMADRS32 { 
    
    self.debug_gaspard("KMADRS32 ");
   return false;
    }if instr &  MASK_KMADS ==  MATCH_KMADS { 
    
    self.debug_gaspard("KMADS ");
   return false;
    }if instr &  MASK_KMADS32 ==  MATCH_KMADS32 { 
    
    self.debug_gaspard("KMADS32 ");
   return false;
    }if instr &  MASK_KMAR64 ==  MATCH_KMAR64 { 
    
    self.debug_gaspard("KMAR64 ");
   return false;
    }if instr &  MASK_KMATT ==  MATCH_KMATT { 
    
    self.debug_gaspard("KMATT ");
   return false;
    }if instr &  MASK_KMATT32 ==  MATCH_KMATT32 { 
    
    self.debug_gaspard("KMATT32 ");
   return false;
    }if instr &  MASK_KMAXDA ==  MATCH_KMAXDA { 
    
    self.debug_gaspard("KMAXDA ");
   return false;
    }if instr &  MASK_KMAXDA32 ==  MATCH_KMAXDA32 { 
    
    self.debug_gaspard("KMAXDA32 ");
   return false;
    }if instr &  MASK_KMAXDS ==  MATCH_KMAXDS { 
    
    self.debug_gaspard("KMAXDS ");
   return false;
    }if instr &  MASK_KMAXDS32 ==  MATCH_KMAXDS32 { 
    
    self.debug_gaspard("KMAXDS32 ");
   return false;
    }if instr &  MASK_KMDA ==  MATCH_KMDA { 
    
    self.debug_gaspard("KMDA ");
   return false;
    }if instr &  MASK_KMDA32 ==  MATCH_KMDA32 { 
    
    self.debug_gaspard("KMDA32 ");
   return false;
    }if instr &  MASK_KMMAC ==  MATCH_KMMAC { 
    
    self.debug_gaspard("KMMAC ");
   return false;
    }if instr &  MASK_KMMAC_U ==  MATCH_KMMAC_U { 
    
    self.debug_gaspard("KMMAC_U ");
   return false;
    }if instr &  MASK_KMMAWB ==  MATCH_KMMAWB { 
    
    self.debug_gaspard("KMMAWB ");
   return false;
    }if instr &  MASK_KMMAWB2 ==  MATCH_KMMAWB2 { 
    
    self.debug_gaspard("KMMAWB2 ");
   return false;
    }if instr &  MASK_KMMAWB2_U ==  MATCH_KMMAWB2_U { 
    
    self.debug_gaspard("KMMAWB2_U ");
   return false;
    }if instr &  MASK_KMMAWB_U ==  MATCH_KMMAWB_U { 
    
    self.debug_gaspard("KMMAWB_U ");
   return false;
    }if instr &  MASK_KMMAWT ==  MATCH_KMMAWT { 
    
    self.debug_gaspard("KMMAWT ");
   return false;
    }if instr &  MASK_KMMAWT2 ==  MATCH_KMMAWT2 { 
    
    self.debug_gaspard("KMMAWT2 ");
   return false;
    }if instr &  MASK_KMMAWT2_U ==  MATCH_KMMAWT2_U { 
    
    self.debug_gaspard("KMMAWT2_U ");
   return false;
    }if instr &  MASK_KMMAWT_U ==  MATCH_KMMAWT_U { 
    
    self.debug_gaspard("KMMAWT_U ");
   return false;
    }if instr &  MASK_KMMSB ==  MATCH_KMMSB { 
    
    self.debug_gaspard("KMMSB ");
   return false;
    }if instr &  MASK_KMMSB_U ==  MATCH_KMMSB_U { 
    
    self.debug_gaspard("KMMSB_U ");
   return false;
    }if instr &  MASK_KMMWB2 ==  MATCH_KMMWB2 { 
    
    self.debug_gaspard("KMMWB2 ");
   return false;
    }if instr &  MASK_KMMWB2_U ==  MATCH_KMMWB2_U { 
    
    self.debug_gaspard("KMMWB2_U ");
   return false;
    }if instr &  MASK_KMMWT2 ==  MATCH_KMMWT2 { 
    
    self.debug_gaspard("KMMWT2 ");
   return false;
    }if instr &  MASK_KMMWT2_U ==  MATCH_KMMWT2_U { 
    
    self.debug_gaspard("KMMWT2_U ");
   return false;
    }if instr &  MASK_KMSDA ==  MATCH_KMSDA { 
    
    self.debug_gaspard("KMSDA ");
   return false;
    }if instr &  MASK_KMSDA32 ==  MATCH_KMSDA32 { 
    
    self.debug_gaspard("KMSDA32 ");
   return false;
    }if instr &  MASK_KMSR64 ==  MATCH_KMSR64 { 
    
    self.debug_gaspard("KMSR64 ");
   return false;
    }if instr &  MASK_KMSXDA ==  MATCH_KMSXDA { 
    
    self.debug_gaspard("KMSXDA ");
   return false;
    }if instr &  MASK_KMSXDA32 ==  MATCH_KMSXDA32 { 
    
    self.debug_gaspard("KMSXDA32 ");
   return false;
    }if instr &  MASK_KMXDA ==  MATCH_KMXDA { 
    
    self.debug_gaspard("KMXDA ");
   return false;
    }if instr &  MASK_KMXDA32 ==  MATCH_KMXDA32 { 
    
    self.debug_gaspard("KMXDA32 ");
   return false;
    }if instr &  MASK_KSLL16 ==  MATCH_KSLL16 { 
    
    self.debug_gaspard("KSLL16 ");
   return false;
    }if instr &  MASK_KSLL32 ==  MATCH_KSLL32 { 
    
    self.debug_gaspard("KSLL32 ");
   return false;
    }if instr &  MASK_KSLL8 ==  MATCH_KSLL8 { 
    
    self.debug_gaspard("KSLL8 ");
   return false;
    }if instr &  MASK_KSLLI16 ==  MATCH_KSLLI16 { 
    
    self.debug_gaspard("KSLLI16 ");
   return false;
    }if instr &  MASK_KSLLI32 ==  MATCH_KSLLI32 { 
    
    self.debug_gaspard("KSLLI32 ");
   return false;
    }if instr &  MASK_KSLLI8 ==  MATCH_KSLLI8 { 
    
    self.debug_gaspard("KSLLI8 ");
   return false;
    }if instr &  MASK_KSLLIW ==  MATCH_KSLLIW { 
    
    self.debug_gaspard("KSLLIW ");
   return false;
    }if instr &  MASK_KSLLW ==  MATCH_KSLLW { 
    
    self.debug_gaspard("KSLLW ");
   return false;
    }if instr &  MASK_KSLRA16 ==  MATCH_KSLRA16 { 
    
    self.debug_gaspard("KSLRA16 ");
   return false;
    }if instr &  MASK_KSLRA16_U ==  MATCH_KSLRA16_U { 
    
    self.debug_gaspard("KSLRA16_U ");
   return false;
    }if instr &  MASK_KSLRA32 ==  MATCH_KSLRA32 { 
    
    self.debug_gaspard("KSLRA32 ");
   return false;
    }if instr &  MASK_KSLRA32_U ==  MATCH_KSLRA32_U { 
    
    self.debug_gaspard("KSLRA32_U ");
   return false;
    }if instr &  MASK_KSLRA8 ==  MATCH_KSLRA8 { 
    
    self.debug_gaspard("KSLRA8 ");
   return false;
    }if instr &  MASK_KSLRA8_U ==  MATCH_KSLRA8_U { 
    
    self.debug_gaspard("KSLRA8_U ");
   return false;
    }if instr &  MASK_KSLRAW ==  MATCH_KSLRAW { 
    
    self.debug_gaspard("KSLRAW ");
   return false;
    }if instr &  MASK_KSLRAW_U ==  MATCH_KSLRAW_U { 
    
    self.debug_gaspard("KSLRAW_U ");
   return false;
    }if instr &  MASK_KSTAS16 ==  MATCH_KSTAS16 { 
    
    self.debug_gaspard("KSTAS16 ");
   return false;
    }if instr &  MASK_KSTAS32 ==  MATCH_KSTAS32 { 
    
    self.debug_gaspard("KSTAS32 ");
   return false;
    }if instr &  MASK_KSTSA16 ==  MATCH_KSTSA16 { 
    
    self.debug_gaspard("KSTSA16 ");
   return false;
    }if instr &  MASK_KSTSA32 ==  MATCH_KSTSA32 { 
    
    self.debug_gaspard("KSTSA32 ");
   return false;
    }if instr &  MASK_KSUB16 ==  MATCH_KSUB16 { 
    
    self.debug_gaspard("KSUB16 ");
   return false;
    }if instr &  MASK_KSUB32 ==  MATCH_KSUB32 { 
    
    self.debug_gaspard("KSUB32 ");
   return false;
    }if instr &  MASK_KSUB64 ==  MATCH_KSUB64 { 
    
    self.debug_gaspard("KSUB64 ");
   return false;
    }if instr &  MASK_KSUB8 ==  MATCH_KSUB8 { 
    
    self.debug_gaspard("KSUB8 ");
   return false;
    }if instr &  MASK_KSUBH ==  MATCH_KSUBH { 
    
    self.debug_gaspard("KSUBH ");
   return false;
    }if instr &  MASK_KSUBW ==  MATCH_KSUBW { 
    
    self.debug_gaspard("KSUBW ");
   return false;
    }if instr &  MASK_KWMMUL ==  MATCH_KWMMUL { 
    
    self.debug_gaspard("KWMMUL ");
   return false;
    }if instr &  MASK_KWMMUL_U ==  MATCH_KWMMUL_U { 
    
    self.debug_gaspard("KWMMUL_U ");
   return false;
    }if instr &  MASK_LB ==  MATCH_LB { 
    
    self.debug_gaspard("LB ");


        let imm = instr >> 20;
        let rs1 = instr >> 15 & 0b1111 ;
    let adress = self.regs[rs1 as usize] + imm as u64 ;
    
    let rd = instr >> 7 & 0b1111;

    let addr_mem  = self.memory_return_u32(adress);


    if addr_mem != None { 
    self.regs[rd as usize] = addr_mem.unwrap()  as u8 as u64;

} else {


    self.regs[rd as usize] = 0;


}




   return false;
    }if instr &  MASK_LB_AQ ==  MATCH_LB_AQ { 
    
    self.debug_gaspard("LB_AQ ");
   return false;
    }if instr &  MASK_LBU ==  MATCH_LBU { 
    
    self.debug_gaspard("LBU ");
   return false;
    }if instr &  MASK_LD ==  MATCH_LD { 
    
    self.debug_gaspard("LD ");
   return false;
    }if instr &  MASK_LD_AQ ==  MATCH_LD_AQ { 
    
    self.debug_gaspard("LD_AQ ");
   return false;
    }if instr &  MASK_LDU ==  MATCH_LDU { 
    
    self.debug_gaspard("LDU ");
   return false;
    }if instr &  MASK_LH ==  MATCH_LH { 
    
    self.debug_gaspard("LH ");

    let imm = instr >> 20;
    let rs1 = instr >> 15 & 0b1111 ;
let adress = self.regs[rs1 as usize] + imm as u64 ;

let rd = instr >> 7 & 0b1111;

let addr_mem  = self.memory_return_u32(adress);


if addr_mem != None { 
self.regs[rd as usize] = addr_mem.unwrap() as u16 as u64;

} else {


self.regs[rd as usize] = 0;


}







   return false;
    }if instr &  MASK_LH_AQ ==  MATCH_LH_AQ { 
    
    self.debug_gaspard("LH_AQ ");
   return false;
    }if instr &  MASK_LHU ==  MATCH_LHU { 
    
    self.debug_gaspard("LHU ");






    let imm = instr >> 20;
    let rs1 = instr >> 15 & 0b1111 ;
let adress = self.regs[rs1 as usize] + imm as u64 ;

let rd = instr >> 7 & 0b1111;

let addr_mem  = self.memory_return_u32(adress);


if addr_mem != None { 
self.regs[rd as usize] = addr_mem.unwrap() as u16 as u64;

} else {


self.regs[rd as usize] = 0;


}



   return false;
    }if instr &  MASK_LPAD ==  MATCH_LPAD { 
    
    self.debug_gaspard("LPAD ");
   return false;
    }if instr &  MASK_LQ ==  MATCH_LQ { 
    
    self.debug_gaspard("LQ ");
   return false;
    }if instr &  MASK_LR_D ==  MATCH_LR_D { 
    
    self.debug_gaspard("LR_D ");
   return false;
    }if instr &  MASK_LR_W ==  MATCH_LR_W { 
    
    self.debug_gaspard("LR_W ");
   return false;
    }if instr &  MASK_LUI ==  MATCH_LUI { 
    
    self.debug_gaspard("LUI ");



    self.clear_flags();

    let imm : u32 = instr >> 12 ;
    let rd : u32 = instr>>7 & 0b11111;



    self.regs[rd as usize] |= (imm << 12) as u64;


   return false;
    }if instr &  MASK_LW ==  MATCH_LW { 
    
    self.debug_gaspard("LW ");




    let imm = instr >> 20;
    let rs1 = instr >> 15 & 0b1111 ;
let adress = self.regs[rs1 as usize] + imm as u64 ;

let rd = instr >> 7 & 0b1111;

let addr_mem  = self.memory_return_u32(adress);


if addr_mem != None { 
self.regs[rd as usize] = addr_mem.unwrap() as u64;

} else {


self.regs[rd as usize] = 0;


}







   return false;
    }if instr &  MASK_LW_AQ ==  MATCH_LW_AQ { 
    
    self.debug_gaspard("LW_AQ ");
   return false;
    }if instr &  MASK_LWU ==  MATCH_LWU { 
    
    self.debug_gaspard("LWU ");
   return false;
    }if instr &  MASK_MADDR32 ==  MATCH_MADDR32 { 
    
    self.debug_gaspard("MADDR32 ");
   return false;
    }if instr &  MASK_MAX ==  MATCH_MAX { 
    
    self.debug_gaspard("MAX ");
   return false;
    }if instr &  MASK_MAXU ==  MATCH_MAXU { 
    
    self.debug_gaspard("MAXU ");
   return false;
    }if instr &  MASK_MIN ==  MATCH_MIN { 
    
    self.debug_gaspard("MIN ");
   return false;
    }if instr &  MASK_MINU ==  MATCH_MINU { 
    
    self.debug_gaspard("MINU ");
   return false;
    }if instr &  MASK_MNRET ==  MATCH_MNRET { 
    
    self.debug_gaspard("MNRET ");
   return false;
    }if instr &  MASK_MOP_R_0 ==  MATCH_MOP_R_0 { 
    
    self.debug_gaspard("MOP_R_0 ");
   return false;
    }if instr &  MASK_MOP_R_1 ==  MATCH_MOP_R_1 { 
    
    self.debug_gaspard("MOP_R_1 ");
   return false;
    }if instr &  MASK_MOP_R_10 ==  MATCH_MOP_R_10 { 
    
    self.debug_gaspard("MOP_R_10 ");
   return false;
    }if instr &  MASK_MOP_R_11 ==  MATCH_MOP_R_11 { 
    
    self.debug_gaspard("MOP_R_11 ");
   return false;
    }if instr &  MASK_MOP_R_12 ==  MATCH_MOP_R_12 { 
    
    self.debug_gaspard("MOP_R_12 ");
   return false;
    }if instr &  MASK_MOP_R_13 ==  MATCH_MOP_R_13 { 
    
    self.debug_gaspard("MOP_R_13 ");
   return false;
    }if instr &  MASK_MOP_R_14 ==  MATCH_MOP_R_14 { 
    
    self.debug_gaspard("MOP_R_14 ");
   return false;
    }if instr &  MASK_MOP_R_15 ==  MATCH_MOP_R_15 { 
    
    self.debug_gaspard("MOP_R_15 ");
   return false;
    }if instr &  MASK_MOP_R_16 ==  MATCH_MOP_R_16 { 
    
    self.debug_gaspard("MOP_R_16 ");
   return false;
    }if instr &  MASK_MOP_R_17 ==  MATCH_MOP_R_17 { 
    
    self.debug_gaspard("MOP_R_17 ");
   return false;
    }if instr &  MASK_MOP_R_18 ==  MATCH_MOP_R_18 { 
    
    self.debug_gaspard("MOP_R_18 ");
   return false;
    }if instr &  MASK_MOP_R_19 ==  MATCH_MOP_R_19 { 
    
    self.debug_gaspard("MOP_R_19 ");
   return false;
    }if instr &  MASK_MOP_R_2 ==  MATCH_MOP_R_2 { 
    
    self.debug_gaspard("MOP_R_2 ");
   return false;
    }if instr &  MASK_MOP_R_20 ==  MATCH_MOP_R_20 { 
    
    self.debug_gaspard("MOP_R_20 ");
   return false;
    }if instr &  MASK_MOP_R_21 ==  MATCH_MOP_R_21 { 
    
    self.debug_gaspard("MOP_R_21 ");
   return false;
    }if instr &  MASK_MOP_R_22 ==  MATCH_MOP_R_22 { 
    
    self.debug_gaspard("MOP_R_22 ");
   return false;
    }if instr &  MASK_MOP_R_23 ==  MATCH_MOP_R_23 { 
    
    self.debug_gaspard("MOP_R_23 ");
   return false;
    }if instr &  MASK_MOP_R_24 ==  MATCH_MOP_R_24 { 
    
    self.debug_gaspard("MOP_R_24 ");
   return false;
    }if instr &  MASK_MOP_R_25 ==  MATCH_MOP_R_25 { 
    
    self.debug_gaspard("MOP_R_25 ");
   return false;
    }if instr &  MASK_MOP_R_26 ==  MATCH_MOP_R_26 { 
    
    self.debug_gaspard("MOP_R_26 ");
   return false;
    }if instr &  MASK_MOP_R_27 ==  MATCH_MOP_R_27 { 
    
    self.debug_gaspard("MOP_R_27 ");
   return false;
    }if instr &  MASK_MOP_R_28 ==  MATCH_MOP_R_28 { 
    
    self.debug_gaspard("MOP_R_28 ");
   return false;
    }if instr &  MASK_MOP_R_29 ==  MATCH_MOP_R_29 { 
    
    self.debug_gaspard("MOP_R_29 ");
   return false;
    }if instr &  MASK_MOP_R_3 ==  MATCH_MOP_R_3 { 
    
    self.debug_gaspard("MOP_R_3 ");
   return false;
    }if instr &  MASK_MOP_R_30 ==  MATCH_MOP_R_30 { 
    
    self.debug_gaspard("MOP_R_30 ");
   return false;
    }if instr &  MASK_MOP_R_31 ==  MATCH_MOP_R_31 { 
    
    self.debug_gaspard("MOP_R_31 ");
   return false;
    }if instr &  MASK_MOP_R_4 ==  MATCH_MOP_R_4 { 
    
    self.debug_gaspard("MOP_R_4 ");
   return false;
    }if instr &  MASK_MOP_R_5 ==  MATCH_MOP_R_5 { 
    
    self.debug_gaspard("MOP_R_5 ");
   return false;
    }if instr &  MASK_MOP_R_6 ==  MATCH_MOP_R_6 { 
    
    self.debug_gaspard("MOP_R_6 ");
   return false;
    }if instr &  MASK_MOP_R_7 ==  MATCH_MOP_R_7 { 
    
    self.debug_gaspard("MOP_R_7 ");
   return false;
    }if instr &  MASK_MOP_R_8 ==  MATCH_MOP_R_8 { 
    
    self.debug_gaspard("MOP_R_8 ");
   return false;
    }if instr &  MASK_MOP_R_9 ==  MATCH_MOP_R_9 { 
    
    self.debug_gaspard("MOP_R_9 ");
   return false;
    }if instr &  MASK_MOP_R_N ==  MATCH_MOP_R_N { 
    
    self.debug_gaspard("MOP_R_N ");
   return false;
    }if instr &  MASK_MOP_RR_0 ==  MATCH_MOP_RR_0 { 
    
    self.debug_gaspard("MOP_RR_0 ");
   return false;
    }if instr &  MASK_MOP_RR_1 ==  MATCH_MOP_RR_1 { 
    
    self.debug_gaspard("MOP_RR_1 ");
   return false;
    }if instr &  MASK_MOP_RR_2 ==  MATCH_MOP_RR_2 { 
    
    self.debug_gaspard("MOP_RR_2 ");
   return false;
    }if instr &  MASK_MOP_RR_3 ==  MATCH_MOP_RR_3 { 
    
    self.debug_gaspard("MOP_RR_3 ");
   return false;
    }if instr &  MASK_MOP_RR_4 ==  MATCH_MOP_RR_4 { 
    
    self.debug_gaspard("MOP_RR_4 ");
   return false;
    }if instr &  MASK_MOP_RR_5 ==  MATCH_MOP_RR_5 { 
    
    self.debug_gaspard("MOP_RR_5 ");
   return false;
    }if instr &  MASK_MOP_RR_6 ==  MATCH_MOP_RR_6 { 
    
    self.debug_gaspard("MOP_RR_6 ");
   return false;
    }if instr &  MASK_MOP_RR_7 ==  MATCH_MOP_RR_7 { 
    
    self.debug_gaspard("MOP_RR_7 ");
   return false;
    }if instr &  MASK_MOP_RR_N ==  MATCH_MOP_RR_N { 
    
    self.debug_gaspard("MOP_RR_N ");
   return false;
    }if instr &  MASK_MRET ==  MATCH_MRET { 
    
    self.debug_gaspard("MRET ");
   return false;
    }if instr &  MASK_MSUBR32 ==  MATCH_MSUBR32 { 
    
    self.debug_gaspard("MSUBR32 ");
   return false;
    }if instr &  MASK_MUL ==  MATCH_MUL { 
    
    self.debug_gaspard("MUL ");
   return false;
    }if instr &  MASK_MULH ==  MATCH_MULH { 
    
    self.debug_gaspard("MULH ");
   return false;
    }if instr &  MASK_MULHSU ==  MATCH_MULHSU { 
    
    self.debug_gaspard("MULHSU ");
   return false;
    }if instr &  MASK_MULHU ==  MATCH_MULHU { 
    
    self.debug_gaspard("MULHU ");
   return false;
    }if instr &  MASK_MULR64 ==  MATCH_MULR64 { 
    
    self.debug_gaspard("MULR64 ");
   return false;
    }if instr &  MASK_MULSR64 ==  MATCH_MULSR64 { 
    
    self.debug_gaspard("MULSR64 ");
   return false;
    }if instr &  MASK_MULW ==  MATCH_MULW { 
    
    self.debug_gaspard("MULW ");
   return false;
    }if instr &  MASK_NTL_ALL ==  MATCH_NTL_ALL { 
    
    self.debug_gaspard("NTL_ALL ");
   return false;
    }if instr &  MASK_NTL_P1 ==  MATCH_NTL_P1 { 
    
    self.debug_gaspard("NTL_P1 ");
   return false;
    }if instr &  MASK_NTL_PALL ==  MATCH_NTL_PALL { 
    
    self.debug_gaspard("NTL_PALL ");
   return false;
    }if instr &  MASK_NTL_S1 ==  MATCH_NTL_S1 { 
    
    self.debug_gaspard("NTL_S1 ");
   return false;
    }if instr &  MASK_OR ==  MATCH_OR { 
    
    self.debug_gaspard("OR ");



    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
self.regs[rd as usize] = self.regs[rs1 as usize] | self.regs[rs2 as usize];




   return false;
    }if instr &  MASK_ORC_B ==  MATCH_ORC_B { 
    
    self.debug_gaspard("ORC_B ");
   return false;
    }if instr &  MASK_ORI ==  MATCH_ORI { 
    
    self.debug_gaspard("ORI ");




    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let imm = instr >> 20;
    self.clear_flags(); 


    self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);



    self.regs[rd as usize] = self.regs[rs1 as usize] | imm as u64;






   return false;
    }if instr &  MASK_ORN ==  MATCH_ORN { 
    
    self.debug_gaspard("ORN ");
   return false;
    }if instr &  MASK_PACK ==  MATCH_PACK { 
    
    self.debug_gaspard("PACK ");
   return false;
    }if instr &  MASK_PACKH ==  MATCH_PACKH { 
    
    self.debug_gaspard("PACKH ");
   return false;
    }if instr &  MASK_PACKU ==  MATCH_PACKU { 
    
    self.debug_gaspard("PACKU ");
   return false;
    }if instr &  MASK_PACKUW ==  MATCH_PACKUW { 
    
    self.debug_gaspard("PACKUW ");
   return false;
    }if instr &  MASK_PACKW ==  MATCH_PACKW { 
    
    self.debug_gaspard("PACKW ");
   return false;
    }if instr &  MASK_PAUSE ==  MATCH_PAUSE { 
    
    self.debug_gaspard("PAUSE ");
   return false;
    }if instr &  MASK_PBSAD ==  MATCH_PBSAD { 
    
    self.debug_gaspard("PBSAD ");
   return false;
    }if instr &  MASK_PBSADA ==  MATCH_PBSADA { 
    
    self.debug_gaspard("PBSADA ");
   return false;
    }if instr &  MASK_PKBB16 ==  MATCH_PKBB16 { 
    
    self.debug_gaspard("PKBB16 ");
   return false;
    }if instr &  MASK_PKBT16 ==  MATCH_PKBT16 { 
    
    self.debug_gaspard("PKBT16 ");
   return false;
    }if instr &  MASK_PKBT32 ==  MATCH_PKBT32 { 
    
    self.debug_gaspard("PKBT32 ");
   return false;
    }if instr &  MASK_PKTB16 ==  MATCH_PKTB16 { 
    
    self.debug_gaspard("PKTB16 ");
   return false;
    }if instr &  MASK_PKTB32 ==  MATCH_PKTB32 { 
    
    self.debug_gaspard("PKTB32 ");
   return false;
    }if instr &  MASK_PKTT16 ==  MATCH_PKTT16 { 
    
    self.debug_gaspard("PKTT16 ");
   return false;
    }if instr &  MASK_PREFETCH_I ==  MATCH_PREFETCH_I { 
    
    self.debug_gaspard("PREFETCH_I ");
   return false;
    }if instr &  MASK_PREFETCH_R ==  MATCH_PREFETCH_R { 
    
    self.debug_gaspard("PREFETCH_R ");
   return false;
    }if instr &  MASK_PREFETCH_W ==  MATCH_PREFETCH_W { 
    
    self.debug_gaspard("PREFETCH_W ");
   return false;
    }if instr &  MASK_RADD16 ==  MATCH_RADD16 { 
    
    self.debug_gaspard("RADD16 ");
   return false;
    }if instr &  MASK_RADD32 ==  MATCH_RADD32 { 
    
    self.debug_gaspard("RADD32 ");
   return false;
    }if instr &  MASK_RADD64 ==  MATCH_RADD64 { 
    
    self.debug_gaspard("RADD64 ");
   return false;
    }if instr &  MASK_RADD8 ==  MATCH_RADD8 { 
    
    self.debug_gaspard("RADD8 ");
   return false;
    }if instr &  MASK_RADDW ==  MATCH_RADDW { 
    
    self.debug_gaspard("RADDW ");
   return false;
    }if instr &  MASK_RCRAS16 ==  MATCH_RCRAS16 { 
    
    self.debug_gaspard("RCRAS16 ");
   return false;
    }if instr &  MASK_RCRAS32 ==  MATCH_RCRAS32 { 
    
    self.debug_gaspard("RCRAS32 ");
   return false;
    }if instr &  MASK_RCRSA16 ==  MATCH_RCRSA16 { 
    
    self.debug_gaspard("RCRSA16 ");
   return false;
    }if instr &  MASK_RCRSA32 ==  MATCH_RCRSA32 { 
    
    self.debug_gaspard("RCRSA32 ");
   return false;
    }if instr &  MASK_RDCYCLE ==  MATCH_RDCYCLE { 
    
    self.debug_gaspard("RDCYCLE ");
   return false;
    }if instr &  MASK_RDCYCLEH ==  MATCH_RDCYCLEH { 
    
    self.debug_gaspard("RDCYCLEH ");
   return false;
    }if instr &  MASK_RDINSTRET ==  MATCH_RDINSTRET { 
    
    self.debug_gaspard("RDINSTRET ");
   return false;
    }if instr &  MASK_RDINSTRETH ==  MATCH_RDINSTRETH { 
    
    self.debug_gaspard("RDINSTRETH ");
   return false;
    }if instr &  MASK_RDOV ==  MATCH_RDOV { 
    
    self.debug_gaspard("RDOV ");
   return false;
    }if instr &  MASK_RDTIME ==  MATCH_RDTIME { 
    
    self.debug_gaspard("RDTIME ");
   return false;
    }if instr &  MASK_RDTIMEH ==  MATCH_RDTIMEH { 
    
    self.debug_gaspard("RDTIMEH ");
   return false;
    }if instr &  MASK_REM ==  MATCH_REM { 
    
    self.debug_gaspard("REM ");
   return false;
    }if instr &  MASK_REMU ==  MATCH_REMU { 
    
    self.debug_gaspard("REMU ");
   return false;
    }if instr &  MASK_REMUW ==  MATCH_REMUW { 
    
    self.debug_gaspard("REMUW ");
   return false;
    }if instr &  MASK_REMW ==  MATCH_REMW { 
    
    self.debug_gaspard("REMW ");
   return false;
    }if instr &  MASK_REV ==  MATCH_REV { 
    
    self.debug_gaspard("REV ");
   return false;
    }if instr &  MASK_REV8 ==  MATCH_REV8 { 
    
    self.debug_gaspard("REV8 ");
   return false;
    }if instr &  MASK_REV8_H ==  MATCH_REV8_H { 
    
    self.debug_gaspard("REV8_H ");
   return false;
    }if instr &  MASK_REV8_RV32 ==  MATCH_REV8_RV32 { 
    
    self.debug_gaspard("REV8_RV32 ");
   return false;
    }if instr &  MASK_ROL ==  MATCH_ROL { 
    
    self.debug_gaspard("ROL ");
   return false;
    }if instr &  MASK_ROLW ==  MATCH_ROLW { 
    
    self.debug_gaspard("ROLW ");
   return false;
    }if instr &  MASK_ROR ==  MATCH_ROR { 
    
    self.debug_gaspard("ROR ");
   return false;
    }if instr &  MASK_RORI ==  MATCH_RORI { 
    
    self.debug_gaspard("RORI ");
   return false;
    }if instr &  MASK_RORI_RV32 ==  MATCH_RORI_RV32 { 
    
    self.debug_gaspard("RORI_RV32 ");
   return false;
    }if instr &  MASK_RORIW ==  MATCH_RORIW { 
    
    self.debug_gaspard("RORIW ");
   return false;
    }if instr &  MASK_RORW ==  MATCH_RORW { 
    
    self.debug_gaspard("RORW ");
   return false;
    }if instr &  MASK_RSTAS16 ==  MATCH_RSTAS16 { 
    
    self.debug_gaspard("RSTAS16 ");
   return false;
    }if instr &  MASK_RSTAS32 ==  MATCH_RSTAS32 { 
    
    self.debug_gaspard("RSTAS32 ");
   return false;
    }if instr &  MASK_RSTSA16 ==  MATCH_RSTSA16 { 
    
    self.debug_gaspard("RSTSA16 ");
   return false;
    }if instr &  MASK_RSTSA32 ==  MATCH_RSTSA32 { 
    
    self.debug_gaspard("RSTSA32 ");
   return false;
    }if instr &  MASK_RSUB16 ==  MATCH_RSUB16 { 
    
    self.debug_gaspard("RSUB16 ");
   return false;
    }if instr &  MASK_RSUB32 ==  MATCH_RSUB32 { 
    
    self.debug_gaspard("RSUB32 ");
   return false;
    }if instr &  MASK_RSUB64 ==  MATCH_RSUB64 { 
    
    self.debug_gaspard("RSUB64 ");
   return false;
    }if instr &  MASK_RSUB8 ==  MATCH_RSUB8 { 
    
    self.debug_gaspard("RSUB8 ");
   return false;
    }if instr &  MASK_RSUBW ==  MATCH_RSUBW { 
    
    self.debug_gaspard("RSUBW ");
   return false;
    }if instr &  MASK_SB ==  MATCH_SB { 
    
    self.debug_gaspard("SB ");



    let imm1 = instr >> 20;
    let imm2  = instr >> 7 & 0b1111;
    let imm_final = imm1 | imm2 << 5;
    
    let rs1 = instr >> 15 & 0b1111 ;
let adress = self.regs[rs1 as usize] + imm_final as u64 ;

let rs2 = instr >> 20 & 0b1111;

self.memory_write_char(adress,self.regs[rs2 as usize]);




   return false;
    }if instr &  MASK_SB_RL ==  MATCH_SB_RL { 
    
    self.debug_gaspard("SB_RL ");
   return false;
    }if instr &  MASK_SBREAK ==  MATCH_SBREAK { 
    
    self.debug_gaspard("SBREAK ");
   return false;
    }if instr &  MASK_SC_D ==  MATCH_SC_D { 
    
    self.debug_gaspard("SC_D ");
   return false;
    }if instr &  MASK_SC_W ==  MATCH_SC_W { 
    
    self.debug_gaspard("SC_W ");
   return false;
    }if instr &  MASK_SCALL ==  MATCH_SCALL { 
    
    self.debug_gaspard("SCALL ");
   return false;
    }if instr &  MASK_SCLIP16 ==  MATCH_SCLIP16 { 
    
    self.debug_gaspard("SCLIP16 ");
   return false;
    }if instr &  MASK_SCLIP32 ==  MATCH_SCLIP32 { 
    
    self.debug_gaspard("SCLIP32 ");
   return false;
    }if instr &  MASK_SCLIP8 ==  MATCH_SCLIP8 { 
    
    self.debug_gaspard("SCLIP8 ");
   return false;
    }if instr &  MASK_SCMPLE16 ==  MATCH_SCMPLE16 { 
    
    self.debug_gaspard("SCMPLE16 ");
   return false;
    }if instr &  MASK_SCMPLE8 ==  MATCH_SCMPLE8 { 
    
    self.debug_gaspard("SCMPLE8 ");
   return false;
    }if instr &  MASK_SCMPLT16 ==  MATCH_SCMPLT16 { 
    
    self.debug_gaspard("SCMPLT16 ");
   return false;
    }if instr &  MASK_SCMPLT8 ==  MATCH_SCMPLT8 { 
    
    self.debug_gaspard("SCMPLT8 ");
   return false;
    }if instr &  MASK_SD ==  MATCH_SD { 
    
    self.debug_gaspard("SD ");
   return false;
    }if instr &  MASK_SD_RL ==  MATCH_SD_RL { 
    
    self.debug_gaspard("SD_RL ");
   return false;
    }if instr &  MASK_SEXT_B ==  MATCH_SEXT_B { 
    
    self.debug_gaspard("SEXT_B ");
   return false;
    }if instr &  MASK_SEXT_H ==  MATCH_SEXT_H { 
    
    self.debug_gaspard("SEXT_H ");
   return false;
    }if instr &  MASK_SFENCE_INVAL_IR ==  MATCH_SFENCE_INVAL_IR { 
    
    self.debug_gaspard("SFENCE_INVAL_IR ");
   return false;
    }if instr &  MASK_SFENCE_VMA ==  MATCH_SFENCE_VMA { 
    
    self.debug_gaspard("SFENCE_VMA ");
   return false;
    }if instr &  MASK_SFENCE_W_INVAL ==  MATCH_SFENCE_W_INVAL { 
    
    self.debug_gaspard("SFENCE_W_INVAL ");
   return false;
    }if instr &  MASK_SH ==  MATCH_SH { 
    
    self.debug_gaspard("SH ");



    
    let imm1 = instr >> 20;
    let imm2  = instr >> 7 & 0b1111;
    let imm_final = imm1 | imm2 << 5;
    
    let rs1 = instr >> 15 & 0b1111 ;
let adress = self.regs[rs1 as usize] + imm_final as u64 ;

let rs2 = instr >> 20 & 0b1111;

self.memory_write_short(adress,self.regs[rs2 as usize]);

   return false;
    }if instr &  MASK_SH1ADD ==  MATCH_SH1ADD { 
    
    self.debug_gaspard("SH1ADD ");
   return false;
    }if instr &  MASK_SH1ADD_UW ==  MATCH_SH1ADD_UW { 
    
    self.debug_gaspard("SH1ADD_UW ");
   return false;
    }if instr &  MASK_SH2ADD ==  MATCH_SH2ADD { 
    
    self.debug_gaspard("SH2ADD ");
   return false;
    }if instr &  MASK_SH2ADD_UW ==  MATCH_SH2ADD_UW { 
    
    self.debug_gaspard("SH2ADD_UW ");
   return false;
    }if instr &  MASK_SH3ADD ==  MATCH_SH3ADD { 
    
    self.debug_gaspard("SH3ADD ");
   return false;
    }if instr &  MASK_SH3ADD_UW ==  MATCH_SH3ADD_UW { 
    
    self.debug_gaspard("SH3ADD_UW ");
   return false;
    }if instr &  MASK_SH_RL ==  MATCH_SH_RL { 
    
    self.debug_gaspard("SH_RL ");
   return false;
    }if instr &  MASK_SHA256SIG0 ==  MATCH_SHA256SIG0 { 
    
    self.debug_gaspard("SHA256SIG0 ");
   return false;
    }if instr &  MASK_SHA256SIG1 ==  MATCH_SHA256SIG1 { 
    
    self.debug_gaspard("SHA256SIG1 ");
   return false;
    }if instr &  MASK_SHA256SUM0 ==  MATCH_SHA256SUM0 { 
    
    self.debug_gaspard("SHA256SUM0 ");
   return false;
    }if instr &  MASK_SHA256SUM1 ==  MATCH_SHA256SUM1 { 
    
    self.debug_gaspard("SHA256SUM1 ");
   return false;
    }if instr &  MASK_SHA512SIG0 ==  MATCH_SHA512SIG0 { 
    
    self.debug_gaspard("SHA512SIG0 ");
   return false;
    }if instr &  MASK_SHA512SIG0H ==  MATCH_SHA512SIG0H { 
    
    self.debug_gaspard("SHA512SIG0H ");
   return false;
    }if instr &  MASK_SHA512SIG0L ==  MATCH_SHA512SIG0L { 
    
    self.debug_gaspard("SHA512SIG0L ");
   return false;
    }if instr &  MASK_SHA512SIG1 ==  MATCH_SHA512SIG1 { 
    
    self.debug_gaspard("SHA512SIG1 ");
   return false;
    }if instr &  MASK_SHA512SIG1H ==  MATCH_SHA512SIG1H { 
    
    self.debug_gaspard("SHA512SIG1H ");
   return false;
    }if instr &  MASK_SHA512SIG1L ==  MATCH_SHA512SIG1L { 
    
    self.debug_gaspard("SHA512SIG1L ");
   return false;
    }if instr &  MASK_SHA512SUM0 ==  MATCH_SHA512SUM0 { 
    
    self.debug_gaspard("SHA512SUM0 ");
   return false;
    }if instr &  MASK_SHA512SUM0R ==  MATCH_SHA512SUM0R { 
    
    self.debug_gaspard("SHA512SUM0R ");
   return false;
    }if instr &  MASK_SHA512SUM1 ==  MATCH_SHA512SUM1 { 
    
    self.debug_gaspard("SHA512SUM1 ");
   return false;
    }if instr &  MASK_SHA512SUM1R ==  MATCH_SHA512SUM1R { 
    
    self.debug_gaspard("SHA512SUM1R ");
   return false;
    }if instr &  MASK_SHFL ==  MATCH_SHFL { 
    
    self.debug_gaspard("SHFL ");
   return false;
    }if instr &  MASK_SHFLI ==  MATCH_SHFLI { 
    
    self.debug_gaspard("SHFLI ");
   return false;
    }if instr &  MASK_SHFLW ==  MATCH_SHFLW { 
    
    self.debug_gaspard("SHFLW ");
   return false;
    }if instr &  MASK_SINVAL_VMA ==  MATCH_SINVAL_VMA { 
    
    self.debug_gaspard("SINVAL_VMA ");
   return false;
    }if instr &  MASK_SLL ==  MATCH_SLL { 
    
    self.debug_gaspard("SLL ");


    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
 
 
 
    self.regs[rd as usize] = self.regs[rs1 as usize] << self.regs[rs2 as usize];
 


   return false;
    }if instr &  MASK_SLL16 ==  MATCH_SLL16 { 
    
    self.debug_gaspard("SLL16 ");
   return false;
    }if instr &  MASK_SLL32 ==  MATCH_SLL32 { 
    
    self.debug_gaspard("SLL32 ");
   return false;
    }if instr &  MASK_SLL8 ==  MATCH_SLL8 { 
    
    self.debug_gaspard("SLL8 ");
   return false;
    }if instr &  MASK_SLLD ==  MATCH_SLLD { 
    
    self.debug_gaspard("SLLD ");
   return false;
    }if instr &  MASK_SLLI ==  MATCH_SLLI { 
    
    self.debug_gaspard("SLLI ");




    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let imm = instr >> 20;
    self.clear_flags(); 


    self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);



    self.regs[rd as usize] = self.regs[rs1 as usize] << imm;





   return false;
    }if instr &  MASK_SLLI16 ==  MATCH_SLLI16 { 
    
    self.debug_gaspard("SLLI16 ");
   return false;
    }if instr &  MASK_SLLI32 ==  MATCH_SLLI32 { 
    
    self.debug_gaspard("SLLI32 ");
   return false;
    }if instr &  MASK_SLLI8 ==  MATCH_SLLI8 { 
    
    self.debug_gaspard("SLLI8 ");
   return false;
    }if instr &  MASK_SLLI_RV128 ==  MATCH_SLLI_RV128 { 
    
    self.debug_gaspard("SLLI_RV128 ");
   return false;
    }if instr &  MASK_SLLI_RV32 ==  MATCH_SLLI_RV32 { 
    
    self.debug_gaspard("SLLI_RV32 ");
   return false;
    }if instr &  MASK_SLLI_UW ==  MATCH_SLLI_UW { 
    
    self.debug_gaspard("SLLI_UW ");
   return false;
    }if instr &  MASK_SLLID ==  MATCH_SLLID { 
    
    self.debug_gaspard("SLLID ");
   return false;
    }if instr &  MASK_SLLIW ==  MATCH_SLLIW { 
    
    self.debug_gaspard("SLLIW ");
   return false;
    }if instr &  MASK_SLLW ==  MATCH_SLLW { 
    
    self.debug_gaspard("SLLW ");
   return false;
    }if instr &  MASK_SLO ==  MATCH_SLO { 
    
    self.debug_gaspard("SLO ");
   return false;
    }if instr &  MASK_SLOI ==  MATCH_SLOI { 
    
    self.debug_gaspard("SLOI ");
   return false;
    }if instr &  MASK_SLOIW ==  MATCH_SLOIW { 
    
    self.debug_gaspard("SLOIW ");
   return false;
    }if instr &  MASK_SLOW ==  MATCH_SLOW { 
    
    self.debug_gaspard("SLOW ");
   return false;
    }if instr &  MASK_SLT ==  MATCH_SLT { 
    
    self.debug_gaspard("SLT ");


    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
 
        if self.less_than == true {


            self.regs[rd as usize] = 1 ;


        } else {


            self.regs[rd as usize] = 0 ;


        }
 
    





   return false;
    }if instr &  MASK_SLTI ==  MATCH_SLTI { 
    
    self.debug_gaspard("SLTI ");





    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let imm = instr >> 20;
    self.clear_flags(); 


    self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);
    if self.less_than == true{



        self.regs[rd as usize] = 1;
    } else {

        self.regs[rd as usize] = 0;




    }




   return false;
    }if instr &  MASK_SLTIU ==  MATCH_SLTIU { 
    
    self.debug_gaspard("SLTIU ");






    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let imm = instr >> 20;
    self.clear_flags(); 


    self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);
    if self.regs[rs1 as usize] != 0{



        self.regs[rd as usize] = 1;
    } else {

        self.regs[rd as usize] = 0;




    }




   return false;
    }if instr &  MASK_SLTU ==  MATCH_SLTU { 
    
    self.debug_gaspard("SLTU ");
    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
 
        if self.regs[rs2 as usize] != 0 {


            self.regs[rd as usize] = 1 ;


        } else {


            self.regs[rd as usize] = 0 ;


        }




   return false;
    }if instr &  MASK_SM3P0 ==  MATCH_SM3P0 { 
    
    self.debug_gaspard("SM3P0 ");
   return false;
    }if instr &  MASK_SM3P1 ==  MATCH_SM3P1 { 
    
    self.debug_gaspard("SM3P1 ");
   return false;
    }if instr &  MASK_SM4ED ==  MATCH_SM4ED { 
    
    self.debug_gaspard("SM4ED ");
   return false;
    }if instr &  MASK_SM4KS ==  MATCH_SM4KS { 
    
    self.debug_gaspard("SM4KS ");
   return false;
    }if instr &  MASK_SMAL ==  MATCH_SMAL { 
    
    self.debug_gaspard("SMAL ");
   return false;
    }if instr &  MASK_SMALBB ==  MATCH_SMALBB { 
    
    self.debug_gaspard("SMALBB ");
   return false;
    }if instr &  MASK_SMALBT ==  MATCH_SMALBT { 
    
    self.debug_gaspard("SMALBT ");
   return false;
    }if instr &  MASK_SMALDA ==  MATCH_SMALDA { 
    
    self.debug_gaspard("SMALDA ");
   return false;
    }if instr &  MASK_SMALDRS ==  MATCH_SMALDRS { 
    
    self.debug_gaspard("SMALDRS ");
   return false;
    }if instr &  MASK_SMALDS ==  MATCH_SMALDS { 
    
    self.debug_gaspard("SMALDS ");
   return false;
    }if instr &  MASK_SMALTT ==  MATCH_SMALTT { 
    
    self.debug_gaspard("SMALTT ");
   return false;
    }if instr &  MASK_SMALXDA ==  MATCH_SMALXDA { 
    
    self.debug_gaspard("SMALXDA ");
   return false;
    }if instr &  MASK_SMALXDS ==  MATCH_SMALXDS { 
    
    self.debug_gaspard("SMALXDS ");
   return false;
    }if instr &  MASK_SMAQA ==  MATCH_SMAQA { 
    
    self.debug_gaspard("SMAQA ");
   return false;
    }if instr &  MASK_SMAQA_SU ==  MATCH_SMAQA_SU { 
    
    self.debug_gaspard("SMAQA_SU ");
   return false;
    }if instr &  MASK_SMAR64 ==  MATCH_SMAR64 { 
    
    self.debug_gaspard("SMAR64 ");
   return false;
    }if instr &  MASK_SMAX16 ==  MATCH_SMAX16 { 
    
    self.debug_gaspard("SMAX16 ");
   return false;
    }if instr &  MASK_SMAX32 ==  MATCH_SMAX32 { 
    
    self.debug_gaspard("SMAX32 ");
   return false;
    }if instr &  MASK_SMAX8 ==  MATCH_SMAX8 { 
    
    self.debug_gaspard("SMAX8 ");
   return false;
    }if instr &  MASK_SMBB16 ==  MATCH_SMBB16 { 
    
    self.debug_gaspard("SMBB16 ");
   return false;
    }if instr &  MASK_SMBT16 ==  MATCH_SMBT16 { 
    
    self.debug_gaspard("SMBT16 ");
   return false;
    }if instr &  MASK_SMBT32 ==  MATCH_SMBT32 { 
    
    self.debug_gaspard("SMBT32 ");
   return false;
    }if instr &  MASK_SMDRS ==  MATCH_SMDRS { 
    
    self.debug_gaspard("SMDRS ");
   return false;
    }if instr &  MASK_SMDRS32 ==  MATCH_SMDRS32 { 
    
    self.debug_gaspard("SMDRS32 ");
   return false;
    }if instr &  MASK_SMDS ==  MATCH_SMDS { 
    
    self.debug_gaspard("SMDS ");
   return false;
    }if instr &  MASK_SMDS32 ==  MATCH_SMDS32 { 
    
    self.debug_gaspard("SMDS32 ");
   return false;
    }if instr &  MASK_SMIN16 ==  MATCH_SMIN16 { 
    
    self.debug_gaspard("SMIN16 ");
   return false;
    }if instr &  MASK_SMIN32 ==  MATCH_SMIN32 { 
    
    self.debug_gaspard("SMIN32 ");
   return false;
    }if instr &  MASK_SMIN8 ==  MATCH_SMIN8 { 
    
    self.debug_gaspard("SMIN8 ");
   return false;
    }if instr &  MASK_SMMUL ==  MATCH_SMMUL { 
    
    self.debug_gaspard("SMMUL ");
   return false;
    }if instr &  MASK_SMMUL_U ==  MATCH_SMMUL_U { 
    
    self.debug_gaspard("SMMUL_U ");
   return false;
    }if instr &  MASK_SMMWB ==  MATCH_SMMWB { 
    
    self.debug_gaspard("SMMWB ");
   return false;
    }if instr &  MASK_SMMWB_U ==  MATCH_SMMWB_U { 
    
    self.debug_gaspard("SMMWB_U ");
   return false;
    }if instr &  MASK_SMMWT ==  MATCH_SMMWT { 
    
    self.debug_gaspard("SMMWT ");
   return false;
    }if instr &  MASK_SMMWT_U ==  MATCH_SMMWT_U { 
    
    self.debug_gaspard("SMMWT_U ");
   return false;
    }if instr &  MASK_SMSLDA ==  MATCH_SMSLDA { 
    
    self.debug_gaspard("SMSLDA ");
   return false;
    }if instr &  MASK_SMSLXDA ==  MATCH_SMSLXDA { 
    
    self.debug_gaspard("SMSLXDA ");
   return false;
    }if instr &  MASK_SMSR64 ==  MATCH_SMSR64 { 
    
    self.debug_gaspard("SMSR64 ");
   return false;
    }if instr &  MASK_SMTT16 ==  MATCH_SMTT16 { 
    
    self.debug_gaspard("SMTT16 ");
   return false;
    }if instr &  MASK_SMTT32 ==  MATCH_SMTT32 { 
    
    self.debug_gaspard("SMTT32 ");
   return false;
    }if instr &  MASK_SMUL16 ==  MATCH_SMUL16 { 
    
    self.debug_gaspard("SMUL16 ");
   return false;
    }if instr &  MASK_SMUL8 ==  MATCH_SMUL8 { 
    
    self.debug_gaspard("SMUL8 ");
   return false;
    }if instr &  MASK_SMULX16 ==  MATCH_SMULX16 { 
    
    self.debug_gaspard("SMULX16 ");
   return false;
    }if instr &  MASK_SMULX8 ==  MATCH_SMULX8 { 
    
    self.debug_gaspard("SMULX8 ");
   return false;
    }if instr &  MASK_SMXDS ==  MATCH_SMXDS { 
    
    self.debug_gaspard("SMXDS ");
   return false;
    }if instr &  MASK_SMXDS32 ==  MATCH_SMXDS32 { 
    
    self.debug_gaspard("SMXDS32 ");
   return false;
    }if instr &  MASK_SQ ==  MATCH_SQ { 
    
    self.debug_gaspard("SQ ");
   return false;
    }if instr &  MASK_SRA ==  MATCH_SRA { 
    
    self.debug_gaspard("SRA ");



    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
self.regs[rd as usize] = self.regs[rs1 as usize] >> self.regs[rs2 as usize];





   return false;
    }if instr &  MASK_SRA16 ==  MATCH_SRA16 { 
    
    self.debug_gaspard("SRA16 ");
   return false;
    }if instr &  MASK_SRA16_U ==  MATCH_SRA16_U { 
    
    self.debug_gaspard("SRA16_U ");
   return false;
    }if instr &  MASK_SRA32 ==  MATCH_SRA32 { 
    
    self.debug_gaspard("SRA32 ");
   return false;
    }if instr &  MASK_SRA32_U ==  MATCH_SRA32_U { 
    
    self.debug_gaspard("SRA32_U ");
   return false;
    }if instr &  MASK_SRA8 ==  MATCH_SRA8 { 
    
    self.debug_gaspard("SRA8 ");
   return false;
    }if instr &  MASK_SRA8_U ==  MATCH_SRA8_U { 
    
    self.debug_gaspard("SRA8_U ");
   return false;
    }if instr &  MASK_SRA_U ==  MATCH_SRA_U { 
    
    self.debug_gaspard("SRA_U ");
   return false;
    }if instr &  MASK_SRAD ==  MATCH_SRAD { 
    
    self.debug_gaspard("SRAD ");
   return false;
    }if instr &  MASK_SRAI ==  MATCH_SRAI { 
    
    self.debug_gaspard("SRAI ");


    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let imm = instr >> 20;
    self.clear_flags(); 


    self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);



    self.regs[rd as usize] = self.regs[rs1 as usize] >> imm;





   return false;
    }if instr &  MASK_SRAI16 ==  MATCH_SRAI16 { 
    
    self.debug_gaspard("SRAI16 ");
   return false;
    }if instr &  MASK_SRAI16_U ==  MATCH_SRAI16_U { 
    
    self.debug_gaspard("SRAI16_U ");
   return false;
    }if instr &  MASK_SRAI32 ==  MATCH_SRAI32 { 
    
    self.debug_gaspard("SRAI32 ");
   return false;
    }if instr &  MASK_SRAI32_U ==  MATCH_SRAI32_U { 
    
    self.debug_gaspard("SRAI32_U ");
   return false;
    }if instr &  MASK_SRAI8 ==  MATCH_SRAI8 { 
    
    self.debug_gaspard("SRAI8 ");
   return false;
    }if instr &  MASK_SRAI8_U ==  MATCH_SRAI8_U { 
    
    self.debug_gaspard("SRAI8_U ");
   return false;
    }if instr &  MASK_SRAI_RV128 ==  MATCH_SRAI_RV128 { 
    
    self.debug_gaspard("SRAI_RV128 ");
   return false;
    }if instr &  MASK_SRAI_RV32 ==  MATCH_SRAI_RV32 { 
    
    self.debug_gaspard("SRAI_RV32 ");
   return false;
    }if instr &  MASK_SRAI_U ==  MATCH_SRAI_U { 
    
    self.debug_gaspard("SRAI_U ");
   return false;
    }if instr &  MASK_SRAID ==  MATCH_SRAID { 
    
    self.debug_gaspard("SRAID ");
   return false;
    }if instr &  MASK_SRAIW ==  MATCH_SRAIW { 
    
    self.debug_gaspard("SRAIW ");
   return false;
    }if instr &  MASK_SRAIW_U ==  MATCH_SRAIW_U { 
    
    self.debug_gaspard("SRAIW_U ");
   return false;
    }if instr &  MASK_SRAW ==  MATCH_SRAW { 
    
    self.debug_gaspard("SRAW ");
   return false;
    }if instr &  MASK_SRET ==  MATCH_SRET { 
    
    self.debug_gaspard("SRET ");
   return false;
    }if instr &  MASK_SRL ==  MATCH_SRL { 
    
    self.debug_gaspard("SRL ");



    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
self.regs[rd as usize] = self.regs[rs1 as usize] >> self.regs[rs2 as usize];





   return false;
    }if instr &  MASK_SRL16 ==  MATCH_SRL16 { 
    
    self.debug_gaspard("SRL16 ");
   return false;
    }if instr &  MASK_SRL16_U ==  MATCH_SRL16_U { 
    
    self.debug_gaspard("SRL16_U ");
   return false;
    }if instr &  MASK_SRL32 ==  MATCH_SRL32 { 
    
    self.debug_gaspard("SRL32 ");
   return false;
    }if instr &  MASK_SRL32_U ==  MATCH_SRL32_U { 
    
    self.debug_gaspard("SRL32_U ");
   return false;
    }if instr &  MASK_SRL8 ==  MATCH_SRL8 { 
    
    self.debug_gaspard("SRL8 ");
   return false;
    }if instr &  MASK_SRL8_U ==  MATCH_SRL8_U { 
    
    self.debug_gaspard("SRL8_U ");
   return false;
    }if instr &  MASK_SRLD ==  MATCH_SRLD { 
    
    self.debug_gaspard("SRLD ");
   return false;
    }if instr &  MASK_SRLI ==  MATCH_SRLI { 
    
    self.debug_gaspard("SRLI ");
   
   
   
    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let imm = instr >> 20;
    self.clear_flags(); 


    self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);



    self.regs[rd as usize] = self.regs[rs1 as usize] >> imm;


   
   
   
    return false;
    }if instr &  MASK_SRLI16 ==  MATCH_SRLI16 { 
    
    self.debug_gaspard("SRLI16 ");
   return false;
    }if instr &  MASK_SRLI16_U ==  MATCH_SRLI16_U { 
    
    self.debug_gaspard("SRLI16_U ");
   return false;
    }if instr &  MASK_SRLI32 ==  MATCH_SRLI32 { 
    
    self.debug_gaspard("SRLI32 ");
   return false;
    }if instr &  MASK_SRLI32_U ==  MATCH_SRLI32_U { 
    
    self.debug_gaspard("SRLI32_U ");
   return false;
    }if instr &  MASK_SRLI8 ==  MATCH_SRLI8 { 
    
    self.debug_gaspard("SRLI8 ");
   return false;
    }if instr &  MASK_SRLI8_U ==  MATCH_SRLI8_U { 
    
    self.debug_gaspard("SRLI8_U ");
   return false;
    }if instr &  MASK_SRLI_RV128 ==  MATCH_SRLI_RV128 { 
    
    self.debug_gaspard("SRLI_RV128 ");
   return false;
    }if instr &  MASK_SRLI_RV32 ==  MATCH_SRLI_RV32 { 
    
    self.debug_gaspard("SRLI_RV32 ");
   return false;
    }if instr &  MASK_SRLID ==  MATCH_SRLID { 
    
    self.debug_gaspard("SRLID ");
   return false;
    }if instr &  MASK_SRLIW ==  MATCH_SRLIW { 
    
    self.debug_gaspard("SRLIW ");
   return false;
    }if instr &  MASK_SRLW ==  MATCH_SRLW { 
    
    self.debug_gaspard("SRLW ");
   return false;
    }if instr &  MASK_SRO ==  MATCH_SRO { 
    
    self.debug_gaspard("SRO ");
   return false;
    }if instr &  MASK_SROI ==  MATCH_SROI { 
    
    self.debug_gaspard("SROI ");
   return false;
    }if instr &  MASK_SROIW ==  MATCH_SROIW { 
    
    self.debug_gaspard("SROIW ");
   return false;
    }if instr &  MASK_SROW ==  MATCH_SROW { 
    
    self.debug_gaspard("SROW ");
   return false;
    }if instr &  MASK_SSAMOSWAP_D ==  MATCH_SSAMOSWAP_D { 
    
    self.debug_gaspard("SSAMOSWAP_D ");
   return false;
    }if instr &  MASK_SSAMOSWAP_W ==  MATCH_SSAMOSWAP_W { 
    
    self.debug_gaspard("SSAMOSWAP_W ");
   return false;
    }if instr &  MASK_SSPOPCHK_X1 ==  MATCH_SSPOPCHK_X1 { 
    
    self.debug_gaspard("SSPOPCHK_X1 ");
   return false;
    }if instr &  MASK_SSPOPCHK_X5 ==  MATCH_SSPOPCHK_X5 { 
    
    self.debug_gaspard("SSPOPCHK_X5 ");
   return false;
    }if instr &  MASK_SSPUSH_X1 ==  MATCH_SSPUSH_X1 { 
    
    self.debug_gaspard("SSPUSH_X1 ");
   return false;
    }if instr &  MASK_SSPUSH_X5 ==  MATCH_SSPUSH_X5 { 
    
    self.debug_gaspard("SSPUSH_X5 ");
   return false;
    }if instr &  MASK_SSRDP ==  MATCH_SSRDP { 
    
    self.debug_gaspard("SSRDP ");
   return false;
    }if instr &  MASK_STAS16 ==  MATCH_STAS16 { 
    
    self.debug_gaspard("STAS16 ");
   return false;
    }if instr &  MASK_STAS32 ==  MATCH_STAS32 { 
    
    self.debug_gaspard("STAS32 ");
   return false;
    }if instr &  MASK_STSA16 ==  MATCH_STSA16 { 
    
    self.debug_gaspard("STSA16 ");
   return false;
    }if instr &  MASK_STSA32 ==  MATCH_STSA32 { 
    
    self.debug_gaspard("STSA32 ");
   return false;
    }if instr &  MASK_SUB ==  MATCH_SUB { 
    
    self.debug_gaspard("SUB ");



    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
 
 
 
    self.regs[rd as usize] = self.regs[rs1 as usize] - self.regs[rs2 as usize];
 
 


   return false;
    }if instr &  MASK_SUB16 ==  MATCH_SUB16 { 
    
    self.debug_gaspard("SUB16 ");
   return false;
    }if instr &  MASK_SUB32 ==  MATCH_SUB32 { 
    
    self.debug_gaspard("SUB32 ");
   return false;
    }if instr &  MASK_SUB64 ==  MATCH_SUB64 { 
    
    self.debug_gaspard("SUB64 ");
   return false;
    }if instr &  MASK_SUB8 ==  MATCH_SUB8 { 
    
    self.debug_gaspard("SUB8 ");
   return false;
    }if instr &  MASK_SUBD ==  MATCH_SUBD { 
    
    self.debug_gaspard("SUBD ");
   return false;
    }if instr &  MASK_SUBW ==  MATCH_SUBW { 
    
    self.debug_gaspard("SUBW ");
   return false;
    }if instr &  MASK_SUNPKD810 ==  MATCH_SUNPKD810 { 
    
    self.debug_gaspard("SUNPKD810 ");
   return false;
    }if instr &  MASK_SUNPKD820 ==  MATCH_SUNPKD820 { 
    
    self.debug_gaspard("SUNPKD820 ");
   return false;
    }if instr &  MASK_SUNPKD830 ==  MATCH_SUNPKD830 { 
    
    self.debug_gaspard("SUNPKD830 ");
   return false;
    }if instr &  MASK_SUNPKD831 ==  MATCH_SUNPKD831 { 
    
    self.debug_gaspard("SUNPKD831 ");
   return false;
    }if instr &  MASK_SUNPKD832 ==  MATCH_SUNPKD832 { 
    
    self.debug_gaspard("SUNPKD832 ");
   return false;
    }if instr &  MASK_SW ==  MATCH_SW { 
    
    self.debug_gaspard("SW ");


    let imm1 = instr >> 20;
    let imm2  = instr >> 7 & 0b1111;
    let imm_final = imm1 | imm2 << 5;
    
    let rs1 = instr >> 15 & 0b1111 ;
let adress = self.regs[rs1 as usize] + imm_final as u64 ;

let rs2 = instr >> 20 & 0b1111;

self.memory_write_long(adress,self.regs[rs2 as usize]);









   return false;
    }if instr &  MASK_SW_RL ==  MATCH_SW_RL { 
    
    self.debug_gaspard("SW_RL ");
   return false;
    }if instr &  MASK_UCLIP16 ==  MATCH_UCLIP16 { 
    
    self.debug_gaspard("UCLIP16 ");
   return false;
    }if instr &  MASK_UCLIP32 ==  MATCH_UCLIP32 { 
    
    self.debug_gaspard("UCLIP32 ");
   return false;
    }if instr &  MASK_UCLIP8 ==  MATCH_UCLIP8 { 
    
    self.debug_gaspard("UCLIP8 ");
   return false;
    }if instr &  MASK_UCMPLE16 ==  MATCH_UCMPLE16 { 
    
    self.debug_gaspard("UCMPLE16 ");
   return false;
    }if instr &  MASK_UCMPLE8 ==  MATCH_UCMPLE8 { 
    
    self.debug_gaspard("UCMPLE8 ");
   return false;
    }if instr &  MASK_UCMPLT16 ==  MATCH_UCMPLT16 { 
    
    self.debug_gaspard("UCMPLT16 ");
   return false;
    }if instr &  MASK_UCMPLT8 ==  MATCH_UCMPLT8 { 
    
    self.debug_gaspard("UCMPLT8 ");
   return false;
    }if instr &  MASK_UKADD16 ==  MATCH_UKADD16 { 
    
    self.debug_gaspard("UKADD16 ");
   return false;
    }if instr &  MASK_UKADD32 ==  MATCH_UKADD32 { 
    
    self.debug_gaspard("UKADD32 ");
   return false;
    }if instr &  MASK_UKADD64 ==  MATCH_UKADD64 { 
    
    self.debug_gaspard("UKADD64 ");
   return false;
    }if instr &  MASK_UKADD8 ==  MATCH_UKADD8 { 
    
    self.debug_gaspard("UKADD8 ");
   return false;
    }if instr &  MASK_UKADDH ==  MATCH_UKADDH { 
    
    self.debug_gaspard("UKADDH ");
   return false;
    }if instr &  MASK_UKADDW ==  MATCH_UKADDW { 
    
    self.debug_gaspard("UKADDW ");
   return false;
    }if instr &  MASK_UKCRAS16 ==  MATCH_UKCRAS16 { 
    
    self.debug_gaspard("UKCRAS16 ");
   return false;
    }if instr &  MASK_UKCRAS32 ==  MATCH_UKCRAS32 { 
    
    self.debug_gaspard("UKCRAS32 ");
   return false;
    }if instr &  MASK_UKCRSA16 ==  MATCH_UKCRSA16 { 
    
    self.debug_gaspard("UKCRSA16 ");
   return false;
    }if instr &  MASK_UKCRSA32 ==  MATCH_UKCRSA32 { 
    
    self.debug_gaspard("UKCRSA32 ");
   return false;
    }if instr &  MASK_UKMAR64 ==  MATCH_UKMAR64 { 
    
    self.debug_gaspard("UKMAR64 ");
   return false;
    }if instr &  MASK_UKMSR64 ==  MATCH_UKMSR64 { 
    
    self.debug_gaspard("UKMSR64 ");
   return false;
    }if instr &  MASK_UKSTAS16 ==  MATCH_UKSTAS16 { 
    
    self.debug_gaspard("UKSTAS16 ");
   return false;
    }if instr &  MASK_UKSTAS32 ==  MATCH_UKSTAS32 { 
    
    self.debug_gaspard("UKSTAS32 ");
   return false;
    }if instr &  MASK_UKSTSA16 ==  MATCH_UKSTSA16 { 
    
    self.debug_gaspard("UKSTSA16 ");
   return false;
    }if instr &  MASK_UKSTSA32 ==  MATCH_UKSTSA32 { 
    
    self.debug_gaspard("UKSTSA32 ");
   return false;
    }if instr &  MASK_UKSUB16 ==  MATCH_UKSUB16 { 
    
    self.debug_gaspard("UKSUB16 ");
   return false;
    }if instr &  MASK_UKSUB32 ==  MATCH_UKSUB32 { 
    
    self.debug_gaspard("UKSUB32 ");
   return false;
    }if instr &  MASK_UKSUB64 ==  MATCH_UKSUB64 { 
    
    self.debug_gaspard("UKSUB64 ");
   return false;
    }if instr &  MASK_UKSUB8 ==  MATCH_UKSUB8 { 
    
    self.debug_gaspard("UKSUB8 ");
   return false;
    }if instr &  MASK_UKSUBH ==  MATCH_UKSUBH { 
    
    self.debug_gaspard("UKSUBH ");
   return false;
    }if instr &  MASK_UKSUBW ==  MATCH_UKSUBW { 
    
    self.debug_gaspard("UKSUBW ");
   return false;
    }if instr &  MASK_UMAQA ==  MATCH_UMAQA { 
    
    self.debug_gaspard("UMAQA ");
   return false;
    }if instr &  MASK_UMAR64 ==  MATCH_UMAR64 { 
    
    self.debug_gaspard("UMAR64 ");
   return false;
    }if instr &  MASK_UMAX16 ==  MATCH_UMAX16 { 
    
    self.debug_gaspard("UMAX16 ");
   return false;
    }if instr &  MASK_UMAX32 ==  MATCH_UMAX32 { 
    
    self.debug_gaspard("UMAX32 ");
   return false;
    }if instr &  MASK_UMAX8 ==  MATCH_UMAX8 { 
    
    self.debug_gaspard("UMAX8 ");
   return false;
    }if instr &  MASK_UMIN16 ==  MATCH_UMIN16 { 
    
    self.debug_gaspard("UMIN16 ");
   return false;
    }if instr &  MASK_UMIN32 ==  MATCH_UMIN32 { 
    
    self.debug_gaspard("UMIN32 ");
   return false;
    }if instr &  MASK_UMIN8 ==  MATCH_UMIN8 { 
    
    self.debug_gaspard("UMIN8 ");
   return false;
    }if instr &  MASK_UMSR64 ==  MATCH_UMSR64 { 
    
    self.debug_gaspard("UMSR64 ");
   return false;
    }if instr &  MASK_UMUL16 ==  MATCH_UMUL16 { 
    
    self.debug_gaspard("UMUL16 ");
   return false;
    }if instr &  MASK_UMUL8 ==  MATCH_UMUL8 { 
    
    self.debug_gaspard("UMUL8 ");
   return false;
    }if instr &  MASK_UMULX16 ==  MATCH_UMULX16 { 
    
    self.debug_gaspard("UMULX16 ");
   return false;
    }if instr &  MASK_UMULX8 ==  MATCH_UMULX8 { 
    
    self.debug_gaspard("UMULX8 ");
   return false;
    }if instr &  MASK_UNSHFL ==  MATCH_UNSHFL { 
    
    self.debug_gaspard("UNSHFL ");
   return false;
    }if instr &  MASK_UNSHFLI ==  MATCH_UNSHFLI { 
    
    self.debug_gaspard("UNSHFLI ");
   return false;
    }if instr &  MASK_UNSHFLW ==  MATCH_UNSHFLW { 
    
    self.debug_gaspard("UNSHFLW ");
   return false;
    }if instr &  MASK_UNZIP ==  MATCH_UNZIP { 
    
    self.debug_gaspard("UNZIP ");
   return false;
    }if instr &  MASK_UNZIP16 ==  MATCH_UNZIP16 { 
    
    self.debug_gaspard("UNZIP16 ");
   return false;
    }if instr &  MASK_UNZIP8 ==  MATCH_UNZIP8 { 
    
    self.debug_gaspard("UNZIP8 ");
   return false;
    }if instr &  MASK_URADD16 ==  MATCH_URADD16 { 
    
    self.debug_gaspard("URADD16 ");
   return false;
    }if instr &  MASK_URADD32 ==  MATCH_URADD32 { 
    
    self.debug_gaspard("URADD32 ");
   return false;
    }if instr &  MASK_URADD64 ==  MATCH_URADD64 { 
    
    self.debug_gaspard("URADD64 ");
   return false;
    }if instr &  MASK_URADD8 ==  MATCH_URADD8 { 
    
    self.debug_gaspard("URADD8 ");
   return false;
    }if instr &  MASK_URADDW ==  MATCH_URADDW { 
    
    self.debug_gaspard("URADDW ");
   return false;
    }if instr &  MASK_URCRAS16 ==  MATCH_URCRAS16 { 
    
    self.debug_gaspard("URCRAS16 ");
   return false;
    }if instr &  MASK_URCRAS32 ==  MATCH_URCRAS32 { 
    
    self.debug_gaspard("URCRAS32 ");
   return false;
    }if instr &  MASK_URCRSA16 ==  MATCH_URCRSA16 { 
    
    self.debug_gaspard("URCRSA16 ");
   return false;
    }if instr &  MASK_URCRSA32 ==  MATCH_URCRSA32 { 
    
    self.debug_gaspard("URCRSA32 ");
   return false;
    }if instr &  MASK_URSTAS16 ==  MATCH_URSTAS16 { 
    
    self.debug_gaspard("URSTAS16 ");
   return false;
    }if instr &  MASK_URSTAS32 ==  MATCH_URSTAS32 { 
    
    self.debug_gaspard("URSTAS32 ");
   return false;
    }if instr &  MASK_URSTSA16 ==  MATCH_URSTSA16 { 
    
    self.debug_gaspard("URSTSA16 ");
   return false;
    }if instr &  MASK_URSTSA32 ==  MATCH_URSTSA32 { 
    
    self.debug_gaspard("URSTSA32 ");
   return false;
    }if instr &  MASK_URSUB16 ==  MATCH_URSUB16 { 
    
    self.debug_gaspard("URSUB16 ");
   return false;
    }if instr &  MASK_URSUB32 ==  MATCH_URSUB32 { 
    
    self.debug_gaspard("URSUB32 ");
   return false;
    }if instr &  MASK_URSUB64 ==  MATCH_URSUB64 { 
    
    self.debug_gaspard("URSUB64 ");
   return false;
    }if instr &  MASK_URSUB8 ==  MATCH_URSUB8 { 
    
    self.debug_gaspard("URSUB8 ");
   return false;
    }if instr &  MASK_URSUBW ==  MATCH_URSUBW { 
    
    self.debug_gaspard("URSUBW ");
   return false;
    }if instr &  MASK_VAADD_VV ==  MATCH_VAADD_VV { 
    
    self.debug_gaspard("VAADD_VV ");
   return false;
    }if instr &  MASK_VAADD_VX ==  MATCH_VAADD_VX { 
    
    self.debug_gaspard("VAADD_VX ");
   return false;
    }if instr &  MASK_VAADDU_VV ==  MATCH_VAADDU_VV { 
    
    self.debug_gaspard("VAADDU_VV ");
   return false;
    }if instr &  MASK_VAADDU_VX ==  MATCH_VAADDU_VX { 
    
    self.debug_gaspard("VAADDU_VX ");
   return false;
    }if instr &  MASK_VADC_VIM ==  MATCH_VADC_VIM { 
    
    self.debug_gaspard("VADC_VIM ");
   return false;
    }if instr &  MASK_VADC_VVM ==  MATCH_VADC_VVM { 
    
    self.debug_gaspard("VADC_VVM ");
   return false;
    }if instr &  MASK_VADC_VXM ==  MATCH_VADC_VXM { 
    
    self.debug_gaspard("VADC_VXM ");
   return false;
    }if instr &  MASK_VADD_VI ==  MATCH_VADD_VI { 
    
    self.debug_gaspard("VADD_VI ");
   return false;
    }if instr &  MASK_VADD_VV ==  MATCH_VADD_VV { 
    
    self.debug_gaspard("VADD_VV ");
   return false;
    }if instr &  MASK_VADD_VX ==  MATCH_VADD_VX { 
    
    self.debug_gaspard("VADD_VX ");
   return false;
    }if instr &  MASK_VAESDF_VS ==  MATCH_VAESDF_VS { 
    
    self.debug_gaspard("VAESDF_VS ");
   return false;
    }if instr &  MASK_VAESDF_VV ==  MATCH_VAESDF_VV { 
    
    self.debug_gaspard("VAESDF_VV ");
   return false;
    }if instr &  MASK_VAESDM_VS ==  MATCH_VAESDM_VS { 
    
    self.debug_gaspard("VAESDM_VS ");
   return false;
    }if instr &  MASK_VAESDM_VV ==  MATCH_VAESDM_VV { 
    
    self.debug_gaspard("VAESDM_VV ");
   return false;
    }if instr &  MASK_VAESEF_VS ==  MATCH_VAESEF_VS { 
    
    self.debug_gaspard("VAESEF_VS ");
   return false;
    }if instr &  MASK_VAESEF_VV ==  MATCH_VAESEF_VV { 
    
    self.debug_gaspard("VAESEF_VV ");
   return false;
    }if instr &  MASK_VAESEM_VS ==  MATCH_VAESEM_VS { 
    
    self.debug_gaspard("VAESEM_VS ");
   return false;
    }if instr &  MASK_VAESEM_VV ==  MATCH_VAESEM_VV { 
    
    self.debug_gaspard("VAESEM_VV ");
   return false;
    }if instr &  MASK_VAESKF1_VI ==  MATCH_VAESKF1_VI { 
    
    self.debug_gaspard("VAESKF1_VI ");
   return false;
    }if instr &  MASK_VAESKF2_VI ==  MATCH_VAESKF2_VI { 
    
    self.debug_gaspard("VAESKF2_VI ");
   return false;
    }if instr &  MASK_VAESZ_VS ==  MATCH_VAESZ_VS { 
    
    self.debug_gaspard("VAESZ_VS ");
   return false;
    }if instr &  MASK_VAND_VI ==  MATCH_VAND_VI { 
    
    self.debug_gaspard("VAND_VI ");
   return false;
    }if instr &  MASK_VAND_VV ==  MATCH_VAND_VV { 
    
    self.debug_gaspard("VAND_VV ");
   return false;
    }if instr &  MASK_VAND_VX ==  MATCH_VAND_VX { 
    
    self.debug_gaspard("VAND_VX ");
   return false;
    }if instr &  MASK_VANDN_VV ==  MATCH_VANDN_VV { 
    
    self.debug_gaspard("VANDN_VV ");
   return false;
    }if instr &  MASK_VANDN_VX ==  MATCH_VANDN_VX { 
    
    self.debug_gaspard("VANDN_VX ");
   return false;
    }if instr &  MASK_VASUB_VV ==  MATCH_VASUB_VV { 
    
    self.debug_gaspard("VASUB_VV ");
   return false;
    }if instr &  MASK_VASUB_VX ==  MATCH_VASUB_VX { 
    
    self.debug_gaspard("VASUB_VX ");
   return false;
    }if instr &  MASK_VASUBU_VV ==  MATCH_VASUBU_VV { 
    
    self.debug_gaspard("VASUBU_VV ");
   return false;
    }if instr &  MASK_VASUBU_VX ==  MATCH_VASUBU_VX { 
    
    self.debug_gaspard("VASUBU_VX ");
   return false;
    }if instr &  MASK_VBREV8_V ==  MATCH_VBREV8_V { 
    
    self.debug_gaspard("VBREV8_V ");
   return false;
    }if instr &  MASK_VBREV_V ==  MATCH_VBREV_V { 
    
    self.debug_gaspard("VBREV_V ");
   return false;
    }if instr &  MASK_VCLMUL_VV ==  MATCH_VCLMUL_VV { 
    
    self.debug_gaspard("VCLMUL_VV ");
   return false;
    }if instr &  MASK_VCLMUL_VX ==  MATCH_VCLMUL_VX { 
    
    self.debug_gaspard("VCLMUL_VX ");
   return false;
    }if instr &  MASK_VCLMULH_VV ==  MATCH_VCLMULH_VV { 
    
    self.debug_gaspard("VCLMULH_VV ");
   return false;
    }if instr &  MASK_VCLMULH_VX ==  MATCH_VCLMULH_VX { 
    
    self.debug_gaspard("VCLMULH_VX ");
   return false;
    }if instr &  MASK_VCLZ_V ==  MATCH_VCLZ_V { 
    
    self.debug_gaspard("VCLZ_V ");
   return false;
    }if instr &  MASK_VCOMPRESS_VM ==  MATCH_VCOMPRESS_VM { 
    
    self.debug_gaspard("VCOMPRESS_VM ");
   return false;
    }if instr &  MASK_VCPOP_M ==  MATCH_VCPOP_M { 
    
    self.debug_gaspard("VCPOP_M ");
   return false;
    }if instr &  MASK_VCPOP_V ==  MATCH_VCPOP_V { 
    
    self.debug_gaspard("VCPOP_V ");
   return false;
    }if instr &  MASK_VCTZ_V ==  MATCH_VCTZ_V { 
    
    self.debug_gaspard("VCTZ_V ");
   return false;
    }if instr &  MASK_VDIV_VV ==  MATCH_VDIV_VV { 
    
    self.debug_gaspard("VDIV_VV ");
   return false;
    }if instr &  MASK_VDIV_VX ==  MATCH_VDIV_VX { 
    
    self.debug_gaspard("VDIV_VX ");
   return false;
    }if instr &  MASK_VDIVU_VV ==  MATCH_VDIVU_VV { 
    
    self.debug_gaspard("VDIVU_VV ");
   return false;
    }if instr &  MASK_VDIVU_VX ==  MATCH_VDIVU_VX { 
    
    self.debug_gaspard("VDIVU_VX ");
   return false;
    }if instr &  MASK_VFADD_VF ==  MATCH_VFADD_VF { 
    
    self.debug_gaspard("VFADD_VF ");
   return false;
    }if instr &  MASK_VFADD_VV ==  MATCH_VFADD_VV { 
    
    self.debug_gaspard("VFADD_VV ");
   return false;
    }if instr &  MASK_VFCLASS_V ==  MATCH_VFCLASS_V { 
    
    self.debug_gaspard("VFCLASS_V ");
   return false;
    }if instr &  MASK_VFCVT_F_X_V ==  MATCH_VFCVT_F_X_V { 
    
    self.debug_gaspard("VFCVT_F_X_V ");
   return false;
    }if instr &  MASK_VFCVT_F_XU_V ==  MATCH_VFCVT_F_XU_V { 
    
    self.debug_gaspard("VFCVT_F_XU_V ");
   return false;
    }if instr &  MASK_VFCVT_RTZ_X_F_V ==  MATCH_VFCVT_RTZ_X_F_V { 
    
    self.debug_gaspard("VFCVT_RTZ_X_F_V ");
   return false;
    }if instr &  MASK_VFCVT_RTZ_XU_F_V ==  MATCH_VFCVT_RTZ_XU_F_V { 
    
    self.debug_gaspard("VFCVT_RTZ_XU_F_V ");
   return false;
    }if instr &  MASK_VFCVT_X_F_V ==  MATCH_VFCVT_X_F_V { 
    
    self.debug_gaspard("VFCVT_X_F_V ");
   return false;
    }if instr &  MASK_VFCVT_XU_F_V ==  MATCH_VFCVT_XU_F_V { 
    
    self.debug_gaspard("VFCVT_XU_F_V ");
   return false;
    }if instr &  MASK_VFDIV_VF ==  MATCH_VFDIV_VF { 
    
    self.debug_gaspard("VFDIV_VF ");
   return false;
    }if instr &  MASK_VFDIV_VV ==  MATCH_VFDIV_VV { 
    
    self.debug_gaspard("VFDIV_VV ");
   return false;
    }if instr &  MASK_VFIRST_M ==  MATCH_VFIRST_M { 
    
    self.debug_gaspard("VFIRST_M ");
   return false;
    }if instr &  MASK_VFMACC_VF ==  MATCH_VFMACC_VF { 
    
    self.debug_gaspard("VFMACC_VF ");
   return false;
    }if instr &  MASK_VFMACC_VV ==  MATCH_VFMACC_VV { 
    
    self.debug_gaspard("VFMACC_VV ");
   return false;
    }if instr &  MASK_VFMADD_VF ==  MATCH_VFMADD_VF { 
    
    self.debug_gaspard("VFMADD_VF ");
   return false;
    }if instr &  MASK_VFMADD_VV ==  MATCH_VFMADD_VV { 
    
    self.debug_gaspard("VFMADD_VV ");
   return false;
    }if instr &  MASK_VFMAX_VF ==  MATCH_VFMAX_VF { 
    
    self.debug_gaspard("VFMAX_VF ");
   return false;
    }if instr &  MASK_VFMAX_VV ==  MATCH_VFMAX_VV { 
    
    self.debug_gaspard("VFMAX_VV ");
   return false;
    }if instr &  MASK_VFMERGE_VFM ==  MATCH_VFMERGE_VFM { 
    
    self.debug_gaspard("VFMERGE_VFM ");
   return false;
    }if instr &  MASK_VFMIN_VF ==  MATCH_VFMIN_VF { 
    
    self.debug_gaspard("VFMIN_VF ");
   return false;
    }if instr &  MASK_VFMIN_VV ==  MATCH_VFMIN_VV { 
    
    self.debug_gaspard("VFMIN_VV ");
   return false;
    }if instr &  MASK_VFMSAC_VF ==  MATCH_VFMSAC_VF { 
    
    self.debug_gaspard("VFMSAC_VF ");
   return false;
    }if instr &  MASK_VFMSAC_VV ==  MATCH_VFMSAC_VV { 
    
    self.debug_gaspard("VFMSAC_VV ");
   return false;
    }if instr &  MASK_VFMSUB_VF ==  MATCH_VFMSUB_VF { 
    
    self.debug_gaspard("VFMSUB_VF ");
   return false;
    }if instr &  MASK_VFMSUB_VV ==  MATCH_VFMSUB_VV { 
    
    self.debug_gaspard("VFMSUB_VV ");
   return false;
    }if instr &  MASK_VFMUL_VF ==  MATCH_VFMUL_VF { 
    
    self.debug_gaspard("VFMUL_VF ");
   return false;
    }if instr &  MASK_VFMUL_VV ==  MATCH_VFMUL_VV { 
    
    self.debug_gaspard("VFMUL_VV ");
   return false;
    }if instr &  MASK_VFMV_F_S ==  MATCH_VFMV_F_S { 
    
    self.debug_gaspard("VFMV_F_S ");
   return false;
    }if instr &  MASK_VFMV_S_F ==  MATCH_VFMV_S_F { 
    
    self.debug_gaspard("VFMV_S_F ");
   return false;
    }if instr &  MASK_VFMV_V_F ==  MATCH_VFMV_V_F { 
    
    self.debug_gaspard("VFMV_V_F ");
   return false;
    }if instr &  MASK_VFNCVT_F_F_W ==  MATCH_VFNCVT_F_F_W { 
    
    self.debug_gaspard("VFNCVT_F_F_W ");
   return false;
    }if instr &  MASK_VFNCVT_F_X_W ==  MATCH_VFNCVT_F_X_W { 
    
    self.debug_gaspard("VFNCVT_F_X_W ");
   return false;
    }if instr &  MASK_VFNCVT_F_XU_W ==  MATCH_VFNCVT_F_XU_W { 
    
    self.debug_gaspard("VFNCVT_F_XU_W ");
   return false;
    }if instr &  MASK_VFNCVT_ROD_F_F_W ==  MATCH_VFNCVT_ROD_F_F_W { 
    
    self.debug_gaspard("VFNCVT_ROD_F_F_W ");
   return false;
    }if instr &  MASK_VFNCVT_RTZ_X_F_W ==  MATCH_VFNCVT_RTZ_X_F_W { 
    
    self.debug_gaspard("VFNCVT_RTZ_X_F_W ");
   return false;
    }if instr &  MASK_VFNCVT_RTZ_XU_F_W ==  MATCH_VFNCVT_RTZ_XU_F_W { 
    
    self.debug_gaspard("VFNCVT_RTZ_XU_F_W ");
   return false;
    }if instr &  MASK_VFNCVT_X_F_W ==  MATCH_VFNCVT_X_F_W { 
    
    self.debug_gaspard("VFNCVT_X_F_W ");
   return false;
    }if instr &  MASK_VFNCVT_XU_F_W ==  MATCH_VFNCVT_XU_F_W { 
    
    self.debug_gaspard("VFNCVT_XU_F_W ");
   return false;
    }if instr &  MASK_VFNCVTBF16_F_F_W ==  MATCH_VFNCVTBF16_F_F_W { 
    
    self.debug_gaspard("VFNCVTBF16_F_F_W ");
   return false;
    }if instr &  MASK_VFNMACC_VF ==  MATCH_VFNMACC_VF { 
    
    self.debug_gaspard("VFNMACC_VF ");
   return false;
    }if instr &  MASK_VFNMACC_VV ==  MATCH_VFNMACC_VV { 
    
    self.debug_gaspard("VFNMACC_VV ");
   return false;
    }if instr &  MASK_VFNMADD_VF ==  MATCH_VFNMADD_VF { 
    
    self.debug_gaspard("VFNMADD_VF ");
   return false;
    }if instr &  MASK_VFNMADD_VV ==  MATCH_VFNMADD_VV { 
    
    self.debug_gaspard("VFNMADD_VV ");
   return false;
    }if instr &  MASK_VFNMSAC_VF ==  MATCH_VFNMSAC_VF { 
    
    self.debug_gaspard("VFNMSAC_VF ");
   return false;
    }if instr &  MASK_VFNMSAC_VV ==  MATCH_VFNMSAC_VV { 
    
    self.debug_gaspard("VFNMSAC_VV ");
   return false;
    }if instr &  MASK_VFNMSUB_VF ==  MATCH_VFNMSUB_VF { 
    
    self.debug_gaspard("VFNMSUB_VF ");
   return false;
    }if instr &  MASK_VFNMSUB_VV ==  MATCH_VFNMSUB_VV { 
    
    self.debug_gaspard("VFNMSUB_VV ");
   return false;
    }if instr &  MASK_VFRDIV_VF ==  MATCH_VFRDIV_VF { 
    
    self.debug_gaspard("VFRDIV_VF ");
   return false;
    }if instr &  MASK_VFREC7_V ==  MATCH_VFREC7_V { 
    
    self.debug_gaspard("VFREC7_V ");
   return false;
    }if instr &  MASK_VFREDMAX_VS ==  MATCH_VFREDMAX_VS { 
    
    self.debug_gaspard("VFREDMAX_VS ");
   return false;
    }if instr &  MASK_VFREDMIN_VS ==  MATCH_VFREDMIN_VS { 
    
    self.debug_gaspard("VFREDMIN_VS ");
   return false;
    }if instr &  MASK_VFREDOSUM_VS ==  MATCH_VFREDOSUM_VS { 
    
    self.debug_gaspard("VFREDOSUM_VS ");
   return false;
    }if instr &  MASK_VFREDSUM_VS ==  MATCH_VFREDSUM_VS { 
    
    self.debug_gaspard("VFREDSUM_VS ");
   return false;
    }if instr &  MASK_VFREDUSUM_VS ==  MATCH_VFREDUSUM_VS { 
    
    self.debug_gaspard("VFREDUSUM_VS ");
   return false;
    }if instr &  MASK_VFRSQRT7_V ==  MATCH_VFRSQRT7_V { 
    
    self.debug_gaspard("VFRSQRT7_V ");
   return false;
    }if instr &  MASK_VFRSUB_VF ==  MATCH_VFRSUB_VF { 
    
    self.debug_gaspard("VFRSUB_VF ");
   return false;
    }if instr &  MASK_VFSGNJ_VF ==  MATCH_VFSGNJ_VF { 
    
    self.debug_gaspard("VFSGNJ_VF ");
   return false;
    }if instr &  MASK_VFSGNJ_VV ==  MATCH_VFSGNJ_VV { 
    
    self.debug_gaspard("VFSGNJ_VV ");
   return false;
    }if instr &  MASK_VFSGNJN_VF ==  MATCH_VFSGNJN_VF { 
    
    self.debug_gaspard("VFSGNJN_VF ");
   return false;
    }if instr &  MASK_VFSGNJN_VV ==  MATCH_VFSGNJN_VV { 
    
    self.debug_gaspard("VFSGNJN_VV ");
   return false;
    }if instr &  MASK_VFSGNJX_VF ==  MATCH_VFSGNJX_VF { 
    
    self.debug_gaspard("VFSGNJX_VF ");
   return false;
    }if instr &  MASK_VFSGNJX_VV ==  MATCH_VFSGNJX_VV { 
    
    self.debug_gaspard("VFSGNJX_VV ");
   return false;
    }if instr &  MASK_VFSLIDE1DOWN_VF ==  MATCH_VFSLIDE1DOWN_VF { 
    
    self.debug_gaspard("VFSLIDE1DOWN_VF ");
   return false;
    }if instr &  MASK_VFSLIDE1UP_VF ==  MATCH_VFSLIDE1UP_VF { 
    
    self.debug_gaspard("VFSLIDE1UP_VF ");
   return false;
    }if instr &  MASK_VFSQRT_V ==  MATCH_VFSQRT_V { 
    
    self.debug_gaspard("VFSQRT_V ");
   return false;
    }if instr &  MASK_VFSUB_VF ==  MATCH_VFSUB_VF { 
    
    self.debug_gaspard("VFSUB_VF ");
   return false;
    }if instr &  MASK_VFSUB_VV ==  MATCH_VFSUB_VV { 
    
    self.debug_gaspard("VFSUB_VV ");
   return false;
    }if instr &  MASK_VFWADD_VF ==  MATCH_VFWADD_VF { 
    
    self.debug_gaspard("VFWADD_VF ");
   return false;
    }if instr &  MASK_VFWADD_VV ==  MATCH_VFWADD_VV { 
    
    self.debug_gaspard("VFWADD_VV ");
   return false;
    }if instr &  MASK_VFWADD_WF ==  MATCH_VFWADD_WF { 
    
    self.debug_gaspard("VFWADD_WF ");
   return false;
    }if instr &  MASK_VFWADD_WV ==  MATCH_VFWADD_WV { 
    
    self.debug_gaspard("VFWADD_WV ");
   return false;
    }if instr &  MASK_VFWCVT_F_F_V ==  MATCH_VFWCVT_F_F_V { 
    
    self.debug_gaspard("VFWCVT_F_F_V ");
   return false;
    }if instr &  MASK_VFWCVT_F_X_V ==  MATCH_VFWCVT_F_X_V { 
    
    self.debug_gaspard("VFWCVT_F_X_V ");
   return false;
    }if instr &  MASK_VFWCVT_F_XU_V ==  MATCH_VFWCVT_F_XU_V { 
    
    self.debug_gaspard("VFWCVT_F_XU_V ");
   return false;
    }if instr &  MASK_VFWCVT_RTZ_X_F_V ==  MATCH_VFWCVT_RTZ_X_F_V { 
    
    self.debug_gaspard("VFWCVT_RTZ_X_F_V ");
   return false;
    }if instr &  MASK_VFWCVT_RTZ_XU_F_V ==  MATCH_VFWCVT_RTZ_XU_F_V { 
    
    self.debug_gaspard("VFWCVT_RTZ_XU_F_V ");
   return false;
    }if instr &  MASK_VFWCVT_X_F_V ==  MATCH_VFWCVT_X_F_V { 
    
    self.debug_gaspard("VFWCVT_X_F_V ");
   return false;
    }if instr &  MASK_VFWCVT_XU_F_V ==  MATCH_VFWCVT_XU_F_V { 
    
    self.debug_gaspard("VFWCVT_XU_F_V ");
   return false;
    }if instr &  MASK_VFWCVTBF16_F_F_V ==  MATCH_VFWCVTBF16_F_F_V { 
    
    self.debug_gaspard("VFWCVTBF16_F_F_V ");
   return false;
    }if instr &  MASK_VFWMACC_VF ==  MATCH_VFWMACC_VF { 
    
    self.debug_gaspard("VFWMACC_VF ");
   return false;
    }if instr &  MASK_VFWMACC_VV ==  MATCH_VFWMACC_VV { 
    
    self.debug_gaspard("VFWMACC_VV ");
   return false;
    }if instr &  MASK_VFWMACCBF16_VF ==  MATCH_VFWMACCBF16_VF { 
    
    self.debug_gaspard("VFWMACCBF16_VF ");
   return false;
    }if instr &  MASK_VFWMACCBF16_VV ==  MATCH_VFWMACCBF16_VV { 
    
    self.debug_gaspard("VFWMACCBF16_VV ");
   return false;
    }if instr &  MASK_VFWMSAC_VF ==  MATCH_VFWMSAC_VF { 
    
    self.debug_gaspard("VFWMSAC_VF ");
   return false;
    }if instr &  MASK_VFWMSAC_VV ==  MATCH_VFWMSAC_VV { 
    
    self.debug_gaspard("VFWMSAC_VV ");
   return false;
    }if instr &  MASK_VFWMUL_VF ==  MATCH_VFWMUL_VF { 
    
    self.debug_gaspard("VFWMUL_VF ");
   return false;
    }if instr &  MASK_VFWMUL_VV ==  MATCH_VFWMUL_VV { 
    
    self.debug_gaspard("VFWMUL_VV ");
   return false;
    }if instr &  MASK_VFWNMACC_VF ==  MATCH_VFWNMACC_VF { 
    
    self.debug_gaspard("VFWNMACC_VF ");
   return false;
    }if instr &  MASK_VFWNMACC_VV ==  MATCH_VFWNMACC_VV { 
    
    self.debug_gaspard("VFWNMACC_VV ");
   return false;
    }if instr &  MASK_VFWNMSAC_VF ==  MATCH_VFWNMSAC_VF { 
    
    self.debug_gaspard("VFWNMSAC_VF ");
   return false;
    }if instr &  MASK_VFWNMSAC_VV ==  MATCH_VFWNMSAC_VV { 
    
    self.debug_gaspard("VFWNMSAC_VV ");
   return false;
    }if instr &  MASK_VFWREDOSUM_VS ==  MATCH_VFWREDOSUM_VS { 
    
    self.debug_gaspard("VFWREDOSUM_VS ");
   return false;
    }if instr &  MASK_VFWREDSUM_VS ==  MATCH_VFWREDSUM_VS { 
    
    self.debug_gaspard("VFWREDSUM_VS ");
   return false;
    }if instr &  MASK_VFWREDUSUM_VS ==  MATCH_VFWREDUSUM_VS { 
    
    self.debug_gaspard("VFWREDUSUM_VS ");
   return false;
    }if instr &  MASK_VFWSUB_VF ==  MATCH_VFWSUB_VF { 
    
    self.debug_gaspard("VFWSUB_VF ");
   return false;
    }if instr &  MASK_VFWSUB_VV ==  MATCH_VFWSUB_VV { 
    
    self.debug_gaspard("VFWSUB_VV ");
   return false;
    }if instr &  MASK_VFWSUB_WF ==  MATCH_VFWSUB_WF { 
    
    self.debug_gaspard("VFWSUB_WF ");
   return false;
    }if instr &  MASK_VFWSUB_WV ==  MATCH_VFWSUB_WV { 
    
    self.debug_gaspard("VFWSUB_WV ");
   return false;
    }if instr &  MASK_VGHSH_VV ==  MATCH_VGHSH_VV { 
    
    self.debug_gaspard("VGHSH_VV ");
   return false;
    }if instr &  MASK_VGMUL_VV ==  MATCH_VGMUL_VV { 
    
    self.debug_gaspard("VGMUL_VV ");
   return false;
    }if instr &  MASK_VID_V ==  MATCH_VID_V { 
    
    self.debug_gaspard("VID_V ");
   return false;
    }if instr &  MASK_VIOTA_M ==  MATCH_VIOTA_M { 
    
    self.debug_gaspard("VIOTA_M ");
   return false;
    }if instr &  MASK_VL1R_V ==  MATCH_VL1R_V { 
    
    self.debug_gaspard("VL1R_V ");
   return false;
    }if instr &  MASK_VL1RE16_V ==  MATCH_VL1RE16_V { 
    
    self.debug_gaspard("VL1RE16_V ");
   return false;
    }if instr &  MASK_VL1RE32_V ==  MATCH_VL1RE32_V { 
    
    self.debug_gaspard("VL1RE32_V ");
   return false;
    }if instr &  MASK_VL1RE64_V ==  MATCH_VL1RE64_V { 
    
    self.debug_gaspard("VL1RE64_V ");
   return false;
    }if instr &  MASK_VL1RE8_V ==  MATCH_VL1RE8_V { 
    
    self.debug_gaspard("VL1RE8_V ");
   return false;
    }if instr &  MASK_VL2R_V ==  MATCH_VL2R_V { 
    
    self.debug_gaspard("VL2R_V ");
   return false;
    }if instr &  MASK_VL2RE16_V ==  MATCH_VL2RE16_V { 
    
    self.debug_gaspard("VL2RE16_V ");
   return false;
    }if instr &  MASK_VL2RE32_V ==  MATCH_VL2RE32_V { 
    
    self.debug_gaspard("VL2RE32_V ");
   return false;
    }if instr &  MASK_VL2RE64_V ==  MATCH_VL2RE64_V { 
    
    self.debug_gaspard("VL2RE64_V ");
   return false;
    }if instr &  MASK_VL2RE8_V ==  MATCH_VL2RE8_V { 
    
    self.debug_gaspard("VL2RE8_V ");
   return false;
    }if instr &  MASK_VL4R_V ==  MATCH_VL4R_V { 
    
    self.debug_gaspard("VL4R_V ");
   return false;
    }if instr &  MASK_VL4RE16_V ==  MATCH_VL4RE16_V { 
    
    self.debug_gaspard("VL4RE16_V ");
   return false;
    }if instr &  MASK_VL4RE32_V ==  MATCH_VL4RE32_V { 
    
    self.debug_gaspard("VL4RE32_V ");
   return false;
    }if instr &  MASK_VL4RE64_V ==  MATCH_VL4RE64_V { 
    
    self.debug_gaspard("VL4RE64_V ");
   return false;
    }if instr &  MASK_VL4RE8_V ==  MATCH_VL4RE8_V { 
    
    self.debug_gaspard("VL4RE8_V ");
   return false;
    }if instr &  MASK_VL8R_V ==  MATCH_VL8R_V { 
    
    self.debug_gaspard("VL8R_V ");
   return false;
    }if instr &  MASK_VL8RE16_V ==  MATCH_VL8RE16_V { 
    
    self.debug_gaspard("VL8RE16_V ");
   return false;
    }if instr &  MASK_VL8RE32_V ==  MATCH_VL8RE32_V { 
    
    self.debug_gaspard("VL8RE32_V ");
   return false;
    }if instr &  MASK_VL8RE64_V ==  MATCH_VL8RE64_V { 
    
    self.debug_gaspard("VL8RE64_V ");
   return false;
    }if instr &  MASK_VL8RE8_V ==  MATCH_VL8RE8_V { 
    
    self.debug_gaspard("VL8RE8_V ");
   return false;
    }if instr &  MASK_VLE1024_V ==  MATCH_VLE1024_V { 
    
    self.debug_gaspard("VLE1024_V ");
   return false;
    }if instr &  MASK_VLE1024FF_V ==  MATCH_VLE1024FF_V { 
    
    self.debug_gaspard("VLE1024FF_V ");
   return false;
    }if instr &  MASK_VLE128_V ==  MATCH_VLE128_V { 
    
    self.debug_gaspard("VLE128_V ");
   return false;
    }if instr &  MASK_VLE128FF_V ==  MATCH_VLE128FF_V { 
    
    self.debug_gaspard("VLE128FF_V ");
   return false;
    }if instr &  MASK_VLE16_V ==  MATCH_VLE16_V { 
    
    self.debug_gaspard("VLE16_V ");
   return false;
    }if instr &  MASK_VLE16FF_V ==  MATCH_VLE16FF_V { 
    
    self.debug_gaspard("VLE16FF_V ");
   return false;
    }if instr &  MASK_VLE1_V ==  MATCH_VLE1_V { 
    
    self.debug_gaspard("VLE1_V ");
   return false;
    }if instr &  MASK_VLE256_V ==  MATCH_VLE256_V { 
    
    self.debug_gaspard("VLE256_V ");
   return false;
    }if instr &  MASK_VLE256FF_V ==  MATCH_VLE256FF_V { 
    
    self.debug_gaspard("VLE256FF_V ");
   return false;
    }if instr &  MASK_VLE32_V ==  MATCH_VLE32_V { 
    
    self.debug_gaspard("VLE32_V ");
   return false;
    }if instr &  MASK_VLE32FF_V ==  MATCH_VLE32FF_V { 
    
    self.debug_gaspard("VLE32FF_V ");
   return false;
    }if instr &  MASK_VLE512_V ==  MATCH_VLE512_V { 
    
    self.debug_gaspard("VLE512_V ");
   return false;
    }if instr &  MASK_VLE512FF_V ==  MATCH_VLE512FF_V { 
    
    self.debug_gaspard("VLE512FF_V ");
   return false;
    }if instr &  MASK_VLE64_V ==  MATCH_VLE64_V { 
    
    self.debug_gaspard("VLE64_V ");
   return false;
    }if instr &  MASK_VLE64FF_V ==  MATCH_VLE64FF_V { 
    
    self.debug_gaspard("VLE64FF_V ");
   return false;
    }if instr &  MASK_VLE8_V ==  MATCH_VLE8_V { 
    
    self.debug_gaspard("VLE8_V ");
   return false;
    }if instr &  MASK_VLE8FF_V ==  MATCH_VLE8FF_V { 
    
    self.debug_gaspard("VLE8FF_V ");
   return false;
    }if instr &  MASK_VLM_V ==  MATCH_VLM_V { 
    
    self.debug_gaspard("VLM_V ");
   return false;
    }if instr &  MASK_VLOXEI1024_V ==  MATCH_VLOXEI1024_V { 
    
    self.debug_gaspard("VLOXEI1024_V ");
   return false;
    }if instr &  MASK_VLOXEI128_V ==  MATCH_VLOXEI128_V { 
    
    self.debug_gaspard("VLOXEI128_V ");
   return false;
    }if instr &  MASK_VLOXEI16_V ==  MATCH_VLOXEI16_V { 
    
    self.debug_gaspard("VLOXEI16_V ");
   return false;
    }if instr &  MASK_VLOXEI256_V ==  MATCH_VLOXEI256_V { 
    
    self.debug_gaspard("VLOXEI256_V ");
   return false;
    }if instr &  MASK_VLOXEI32_V ==  MATCH_VLOXEI32_V { 
    
    self.debug_gaspard("VLOXEI32_V ");
   return false;
    }if instr &  MASK_VLOXEI512_V ==  MATCH_VLOXEI512_V { 
    
    self.debug_gaspard("VLOXEI512_V ");
   return false;
    }if instr &  MASK_VLOXEI64_V ==  MATCH_VLOXEI64_V { 
    
    self.debug_gaspard("VLOXEI64_V ");
   return false;
    }if instr &  MASK_VLOXEI8_V ==  MATCH_VLOXEI8_V { 
    
    self.debug_gaspard("VLOXEI8_V ");
   return false;
    }if instr &  MASK_VLSE1024_V ==  MATCH_VLSE1024_V { 
    
    self.debug_gaspard("VLSE1024_V ");
   return false;
    }if instr &  MASK_VLSE128_V ==  MATCH_VLSE128_V { 
    
    self.debug_gaspard("VLSE128_V ");
   return false;
    }if instr &  MASK_VLSE16_V ==  MATCH_VLSE16_V { 
    
    self.debug_gaspard("VLSE16_V ");
   return false;
    }if instr &  MASK_VLSE256_V ==  MATCH_VLSE256_V { 
    
    self.debug_gaspard("VLSE256_V ");
   return false;
    }if instr &  MASK_VLSE32_V ==  MATCH_VLSE32_V { 
    
    self.debug_gaspard("VLSE32_V ");
   return false;
    }if instr &  MASK_VLSE512_V ==  MATCH_VLSE512_V { 
    
    self.debug_gaspard("VLSE512_V ");
   return false;
    }if instr &  MASK_VLSE64_V ==  MATCH_VLSE64_V { 
    
    self.debug_gaspard("VLSE64_V ");
   return false;
    }if instr &  MASK_VLSE8_V ==  MATCH_VLSE8_V { 
    
    self.debug_gaspard("VLSE8_V ");
   return false;
    }if instr &  MASK_VLUXEI1024_V ==  MATCH_VLUXEI1024_V { 
    
    self.debug_gaspard("VLUXEI1024_V ");
   return false;
    }if instr &  MASK_VLUXEI128_V ==  MATCH_VLUXEI128_V { 
    
    self.debug_gaspard("VLUXEI128_V ");
   return false;
    }if instr &  MASK_VLUXEI16_V ==  MATCH_VLUXEI16_V { 
    
    self.debug_gaspard("VLUXEI16_V ");
   return false;
    }if instr &  MASK_VLUXEI256_V ==  MATCH_VLUXEI256_V { 
    
    self.debug_gaspard("VLUXEI256_V ");
   return false;
    }if instr &  MASK_VLUXEI32_V ==  MATCH_VLUXEI32_V { 
    
    self.debug_gaspard("VLUXEI32_V ");
   return false;
    }if instr &  MASK_VLUXEI512_V ==  MATCH_VLUXEI512_V { 
    
    self.debug_gaspard("VLUXEI512_V ");
   return false;
    }if instr &  MASK_VLUXEI64_V ==  MATCH_VLUXEI64_V { 
    
    self.debug_gaspard("VLUXEI64_V ");
   return false;
    }if instr &  MASK_VLUXEI8_V ==  MATCH_VLUXEI8_V { 
    
    self.debug_gaspard("VLUXEI8_V ");
   return false;
    }if instr &  MASK_VMACC_VV ==  MATCH_VMACC_VV { 
    
    self.debug_gaspard("VMACC_VV ");
   return false;
    }if instr &  MASK_VMACC_VX ==  MATCH_VMACC_VX { 
    
    self.debug_gaspard("VMACC_VX ");
   return false;
    }if instr &  MASK_VMADC_VI ==  MATCH_VMADC_VI { 
    
    self.debug_gaspard("VMADC_VI ");
   return false;
    }if instr &  MASK_VMADC_VIM ==  MATCH_VMADC_VIM { 
    
    self.debug_gaspard("VMADC_VIM ");
   return false;
    }if instr &  MASK_VMADC_VV ==  MATCH_VMADC_VV { 
    
    self.debug_gaspard("VMADC_VV ");
   return false;
    }if instr &  MASK_VMADC_VVM ==  MATCH_VMADC_VVM { 
    
    self.debug_gaspard("VMADC_VVM ");
   return false;
    }if instr &  MASK_VMADC_VX ==  MATCH_VMADC_VX { 
    
    self.debug_gaspard("VMADC_VX ");
   return false;
    }if instr &  MASK_VMADC_VXM ==  MATCH_VMADC_VXM { 
    
    self.debug_gaspard("VMADC_VXM ");
   return false;
    }if instr &  MASK_VMADD_VV ==  MATCH_VMADD_VV { 
    
    self.debug_gaspard("VMADD_VV ");
   return false;
    }if instr &  MASK_VMADD_VX ==  MATCH_VMADD_VX { 
    
    self.debug_gaspard("VMADD_VX ");
   return false;
    }if instr &  MASK_VMAND_MM ==  MATCH_VMAND_MM { 
    
    self.debug_gaspard("VMAND_MM ");
   return false;
    }if instr &  MASK_VMANDN_MM ==  MATCH_VMANDN_MM { 
    
    self.debug_gaspard("VMANDN_MM ");
   return false;
    }if instr &  MASK_VMANDNOT_MM ==  MATCH_VMANDNOT_MM { 
    
    self.debug_gaspard("VMANDNOT_MM ");
   return false;
    }if instr &  MASK_VMAX_VV ==  MATCH_VMAX_VV { 
    
    self.debug_gaspard("VMAX_VV ");
   return false;
    }if instr &  MASK_VMAX_VX ==  MATCH_VMAX_VX { 
    
    self.debug_gaspard("VMAX_VX ");
   return false;
    }if instr &  MASK_VMAXU_VV ==  MATCH_VMAXU_VV { 
    
    self.debug_gaspard("VMAXU_VV ");
   return false;
    }if instr &  MASK_VMAXU_VX ==  MATCH_VMAXU_VX { 
    
    self.debug_gaspard("VMAXU_VX ");
   return false;
    }if instr &  MASK_VMERGE_VIM ==  MATCH_VMERGE_VIM { 
    
    self.debug_gaspard("VMERGE_VIM ");
   return false;
    }if instr &  MASK_VMERGE_VVM ==  MATCH_VMERGE_VVM { 
    
    self.debug_gaspard("VMERGE_VVM ");
   return false;
    }if instr &  MASK_VMERGE_VXM ==  MATCH_VMERGE_VXM { 
    
    self.debug_gaspard("VMERGE_VXM ");
   return false;
    }if instr &  MASK_VMFEQ_VF ==  MATCH_VMFEQ_VF { 
    
    self.debug_gaspard("VMFEQ_VF ");
   return false;
    }if instr &  MASK_VMFEQ_VV ==  MATCH_VMFEQ_VV { 
    
    self.debug_gaspard("VMFEQ_VV ");
   return false;
    }if instr &  MASK_VMFGE_VF ==  MATCH_VMFGE_VF { 
    
    self.debug_gaspard("VMFGE_VF ");
   return false;
    }if instr &  MASK_VMFGT_VF ==  MATCH_VMFGT_VF { 
    
    self.debug_gaspard("VMFGT_VF ");
   return false;
    }if instr &  MASK_VMFLE_VF ==  MATCH_VMFLE_VF { 
    
    self.debug_gaspard("VMFLE_VF ");
   return false;
    }if instr &  MASK_VMFLE_VV ==  MATCH_VMFLE_VV { 
    
    self.debug_gaspard("VMFLE_VV ");
   return false;
    }if instr &  MASK_VMFLT_VF ==  MATCH_VMFLT_VF { 
    
    self.debug_gaspard("VMFLT_VF ");
   return false;
    }if instr &  MASK_VMFLT_VV ==  MATCH_VMFLT_VV { 
    
    self.debug_gaspard("VMFLT_VV ");
   return false;
    }if instr &  MASK_VMFNE_VF ==  MATCH_VMFNE_VF { 
    
    self.debug_gaspard("VMFNE_VF ");
   return false;
    }if instr &  MASK_VMFNE_VV ==  MATCH_VMFNE_VV { 
    
    self.debug_gaspard("VMFNE_VV ");
   return false;
    }if instr &  MASK_VMIN_VV ==  MATCH_VMIN_VV { 
    
    self.debug_gaspard("VMIN_VV ");
   return false;
    }if instr &  MASK_VMIN_VX ==  MATCH_VMIN_VX { 
    
    self.debug_gaspard("VMIN_VX ");
   return false;
    }if instr &  MASK_VMINU_VV ==  MATCH_VMINU_VV { 
    
    self.debug_gaspard("VMINU_VV ");
   return false;
    }if instr &  MASK_VMINU_VX ==  MATCH_VMINU_VX { 
    
    self.debug_gaspard("VMINU_VX ");
   return false;
    }if instr &  MASK_VMNAND_MM ==  MATCH_VMNAND_MM { 
    
    self.debug_gaspard("VMNAND_MM ");
   return false;
    }if instr &  MASK_VMNOR_MM ==  MATCH_VMNOR_MM { 
    
    self.debug_gaspard("VMNOR_MM ");
   return false;
    }if instr &  MASK_VMOR_MM ==  MATCH_VMOR_MM { 
    
    self.debug_gaspard("VMOR_MM ");
   return false;
    }if instr &  MASK_VMORN_MM ==  MATCH_VMORN_MM { 
    
    self.debug_gaspard("VMORN_MM ");
   return false;
    }if instr &  MASK_VMORNOT_MM ==  MATCH_VMORNOT_MM { 
    
    self.debug_gaspard("VMORNOT_MM ");
   return false;
    }if instr &  MASK_VMSBC_VV ==  MATCH_VMSBC_VV { 
    
    self.debug_gaspard("VMSBC_VV ");
   return false;
    }if instr &  MASK_VMSBC_VVM ==  MATCH_VMSBC_VVM { 
    
    self.debug_gaspard("VMSBC_VVM ");
   return false;
    }if instr &  MASK_VMSBC_VX ==  MATCH_VMSBC_VX { 
    
    self.debug_gaspard("VMSBC_VX ");
   return false;
    }if instr &  MASK_VMSBC_VXM ==  MATCH_VMSBC_VXM { 
    
    self.debug_gaspard("VMSBC_VXM ");
   return false;
    }if instr &  MASK_VMSBF_M ==  MATCH_VMSBF_M { 
    
    self.debug_gaspard("VMSBF_M ");
   return false;
    }if instr &  MASK_VMSEQ_VI ==  MATCH_VMSEQ_VI { 
    
    self.debug_gaspard("VMSEQ_VI ");
   return false;
    }if instr &  MASK_VMSEQ_VV ==  MATCH_VMSEQ_VV { 
    
    self.debug_gaspard("VMSEQ_VV ");
   return false;
    }if instr &  MASK_VMSEQ_VX ==  MATCH_VMSEQ_VX { 
    
    self.debug_gaspard("VMSEQ_VX ");
   return false;
    }if instr &  MASK_VMSGT_VI ==  MATCH_VMSGT_VI { 
    
    self.debug_gaspard("VMSGT_VI ");
   return false;
    }if instr &  MASK_VMSGT_VX ==  MATCH_VMSGT_VX { 
    
    self.debug_gaspard("VMSGT_VX ");
   return false;
    }if instr &  MASK_VMSGTU_VI ==  MATCH_VMSGTU_VI { 
    
    self.debug_gaspard("VMSGTU_VI ");
   return false;
    }if instr &  MASK_VMSGTU_VX ==  MATCH_VMSGTU_VX { 
    
    self.debug_gaspard("VMSGTU_VX ");
   return false;
    }if instr &  MASK_VMSIF_M ==  MATCH_VMSIF_M { 
    
    self.debug_gaspard("VMSIF_M ");
   return false;
    }if instr &  MASK_VMSLE_VI ==  MATCH_VMSLE_VI { 
    
    self.debug_gaspard("VMSLE_VI ");
   return false;
    }if instr &  MASK_VMSLE_VV ==  MATCH_VMSLE_VV { 
    
    self.debug_gaspard("VMSLE_VV ");
   return false;
    }if instr &  MASK_VMSLE_VX ==  MATCH_VMSLE_VX { 
    
    self.debug_gaspard("VMSLE_VX ");
   return false;
    }if instr &  MASK_VMSLEU_VI ==  MATCH_VMSLEU_VI { 
    
    self.debug_gaspard("VMSLEU_VI ");
   return false;
    }if instr &  MASK_VMSLEU_VV ==  MATCH_VMSLEU_VV { 
    
    self.debug_gaspard("VMSLEU_VV ");
   return false;
    }if instr &  MASK_VMSLEU_VX ==  MATCH_VMSLEU_VX { 
    
    self.debug_gaspard("VMSLEU_VX ");
   return false;
    }if instr &  MASK_VMSLT_VV ==  MATCH_VMSLT_VV { 
    
    self.debug_gaspard("VMSLT_VV ");
   return false;
    }if instr &  MASK_VMSLT_VX ==  MATCH_VMSLT_VX { 
    
    self.debug_gaspard("VMSLT_VX ");
   return false;
    }if instr &  MASK_VMSLTU_VV ==  MATCH_VMSLTU_VV { 
    
    self.debug_gaspard("VMSLTU_VV ");
   return false;
    }if instr &  MASK_VMSLTU_VX ==  MATCH_VMSLTU_VX { 
    
    self.debug_gaspard("VMSLTU_VX ");
   return false;
    }if instr &  MASK_VMSNE_VI ==  MATCH_VMSNE_VI { 
    
    self.debug_gaspard("VMSNE_VI ");
   return false;
    }if instr &  MASK_VMSNE_VV ==  MATCH_VMSNE_VV { 
    
    self.debug_gaspard("VMSNE_VV ");
   return false;
    }if instr &  MASK_VMSNE_VX ==  MATCH_VMSNE_VX { 
    
    self.debug_gaspard("VMSNE_VX ");
   return false;
    }if instr &  MASK_VMSOF_M ==  MATCH_VMSOF_M { 
    
    self.debug_gaspard("VMSOF_M ");
   return false;
    }if instr &  MASK_VMUL_VV ==  MATCH_VMUL_VV { 
    
    self.debug_gaspard("VMUL_VV ");
   return false;
    }if instr &  MASK_VMUL_VX ==  MATCH_VMUL_VX { 
    
    self.debug_gaspard("VMUL_VX ");
   return false;
    }if instr &  MASK_VMULH_VV ==  MATCH_VMULH_VV { 
    
    self.debug_gaspard("VMULH_VV ");
   return false;
    }if instr &  MASK_VMULH_VX ==  MATCH_VMULH_VX { 
    
    self.debug_gaspard("VMULH_VX ");
   return false;
    }if instr &  MASK_VMULHSU_VV ==  MATCH_VMULHSU_VV { 
    
    self.debug_gaspard("VMULHSU_VV ");
   return false;
    }if instr &  MASK_VMULHSU_VX ==  MATCH_VMULHSU_VX { 
    
    self.debug_gaspard("VMULHSU_VX ");
   return false;
    }if instr &  MASK_VMULHU_VV ==  MATCH_VMULHU_VV { 
    
    self.debug_gaspard("VMULHU_VV ");
   return false;
    }if instr &  MASK_VMULHU_VX ==  MATCH_VMULHU_VX { 
    
    self.debug_gaspard("VMULHU_VX ");
   return false;
    }if instr &  MASK_VMV1R_V ==  MATCH_VMV1R_V { 
    
    self.debug_gaspard("VMV1R_V ");
   return false;
    }if instr &  MASK_VMV2R_V ==  MATCH_VMV2R_V { 
    
    self.debug_gaspard("VMV2R_V ");
   return false;
    }if instr &  MASK_VMV4R_V ==  MATCH_VMV4R_V { 
    
    self.debug_gaspard("VMV4R_V ");
   return false;
    }if instr &  MASK_VMV8R_V ==  MATCH_VMV8R_V { 
    
    self.debug_gaspard("VMV8R_V ");
   return false;
    }if instr &  MASK_VMV_S_X ==  MATCH_VMV_S_X { 
    
    self.debug_gaspard("VMV_S_X ");
   return false;
    }if instr &  MASK_VMV_V_I ==  MATCH_VMV_V_I { 
    
    self.debug_gaspard("VMV_V_I ");
   return false;
    }if instr &  MASK_VMV_V_V ==  MATCH_VMV_V_V { 
    
    self.debug_gaspard("VMV_V_V ");
   return false;
    }if instr &  MASK_VMV_V_X ==  MATCH_VMV_V_X { 
    
    self.debug_gaspard("VMV_V_X ");
   return false;
    }if instr &  MASK_VMV_X_S ==  MATCH_VMV_X_S { 
    
    self.debug_gaspard("VMV_X_S ");
   return false;
    }if instr &  MASK_VMXNOR_MM ==  MATCH_VMXNOR_MM { 
    
    self.debug_gaspard("VMXNOR_MM ");
   return false;
    }if instr &  MASK_VMXOR_MM ==  MATCH_VMXOR_MM { 
    
    self.debug_gaspard("VMXOR_MM ");
   return false;
    }if instr &  MASK_VNCLIP_WI ==  MATCH_VNCLIP_WI { 
    
    self.debug_gaspard("VNCLIP_WI ");
   return false;
    }if instr &  MASK_VNCLIP_WV ==  MATCH_VNCLIP_WV { 
    
    self.debug_gaspard("VNCLIP_WV ");
   return false;
    }if instr &  MASK_VNCLIP_WX ==  MATCH_VNCLIP_WX { 
    
    self.debug_gaspard("VNCLIP_WX ");
   return false;
    }if instr &  MASK_VNCLIPU_WI ==  MATCH_VNCLIPU_WI { 
    
    self.debug_gaspard("VNCLIPU_WI ");
   return false;
    }if instr &  MASK_VNCLIPU_WV ==  MATCH_VNCLIPU_WV { 
    
    self.debug_gaspard("VNCLIPU_WV ");
   return false;
    }if instr &  MASK_VNCLIPU_WX ==  MATCH_VNCLIPU_WX { 
    
    self.debug_gaspard("VNCLIPU_WX ");
   return false;
    }if instr &  MASK_VNMSAC_VV ==  MATCH_VNMSAC_VV { 
    
    self.debug_gaspard("VNMSAC_VV ");
   return false;
    }if instr &  MASK_VNMSAC_VX ==  MATCH_VNMSAC_VX { 
    
    self.debug_gaspard("VNMSAC_VX ");
   return false;
    }if instr &  MASK_VNMSUB_VV ==  MATCH_VNMSUB_VV { 
    
    self.debug_gaspard("VNMSUB_VV ");
   return false;
    }if instr &  MASK_VNMSUB_VX ==  MATCH_VNMSUB_VX { 
    
    self.debug_gaspard("VNMSUB_VX ");
   return false;
    }if instr &  MASK_VNSRA_WI ==  MATCH_VNSRA_WI { 
    
    self.debug_gaspard("VNSRA_WI ");
   return false;
    }if instr &  MASK_VNSRA_WV ==  MATCH_VNSRA_WV { 
    
    self.debug_gaspard("VNSRA_WV ");
   return false;
    }if instr &  MASK_VNSRA_WX ==  MATCH_VNSRA_WX { 
    
    self.debug_gaspard("VNSRA_WX ");
   return false;
    }if instr &  MASK_VNSRL_WI ==  MATCH_VNSRL_WI { 
    
    self.debug_gaspard("VNSRL_WI ");
   return false;
    }if instr &  MASK_VNSRL_WV ==  MATCH_VNSRL_WV { 
    
    self.debug_gaspard("VNSRL_WV ");
   return false;
    }if instr &  MASK_VNSRL_WX ==  MATCH_VNSRL_WX { 
    
    self.debug_gaspard("VNSRL_WX ");
   return false;
    }if instr &  MASK_VOR_VI ==  MATCH_VOR_VI { 
    
    self.debug_gaspard("VOR_VI ");
   return false;
    }if instr &  MASK_VOR_VV ==  MATCH_VOR_VV { 
    
    self.debug_gaspard("VOR_VV ");
   return false;
    }if instr &  MASK_VOR_VX ==  MATCH_VOR_VX { 
    
    self.debug_gaspard("VOR_VX ");
   return false;
    }if instr &  MASK_VPOPC_M ==  MATCH_VPOPC_M { 
    
    self.debug_gaspard("VPOPC_M ");
   return false;
    }if instr &  MASK_VREDAND_VS ==  MATCH_VREDAND_VS { 
    
    self.debug_gaspard("VREDAND_VS ");
   return false;
    }if instr &  MASK_VREDMAX_VS ==  MATCH_VREDMAX_VS { 
    
    self.debug_gaspard("VREDMAX_VS ");
   return false;
    }if instr &  MASK_VREDMAXU_VS ==  MATCH_VREDMAXU_VS { 
    
    self.debug_gaspard("VREDMAXU_VS ");
   return false;
    }if instr &  MASK_VREDMIN_VS ==  MATCH_VREDMIN_VS { 
    
    self.debug_gaspard("VREDMIN_VS ");
   return false;
    }if instr &  MASK_VREDMINU_VS ==  MATCH_VREDMINU_VS { 
    
    self.debug_gaspard("VREDMINU_VS ");
   return false;
    }if instr &  MASK_VREDOR_VS ==  MATCH_VREDOR_VS { 
    
    self.debug_gaspard("VREDOR_VS ");
   return false;
    }if instr &  MASK_VREDSUM_VS ==  MATCH_VREDSUM_VS { 
    
    self.debug_gaspard("VREDSUM_VS ");
   return false;
    }if instr &  MASK_VREDXOR_VS ==  MATCH_VREDXOR_VS { 
    
    self.debug_gaspard("VREDXOR_VS ");
   return false;
    }if instr &  MASK_VREM_VV ==  MATCH_VREM_VV { 
    
    self.debug_gaspard("VREM_VV ");
   return false;
    }if instr &  MASK_VREM_VX ==  MATCH_VREM_VX { 
    
    self.debug_gaspard("VREM_VX ");
   return false;
    }if instr &  MASK_VREMU_VV ==  MATCH_VREMU_VV { 
    
    self.debug_gaspard("VREMU_VV ");
   return false;
    }if instr &  MASK_VREMU_VX ==  MATCH_VREMU_VX { 
    
    self.debug_gaspard("VREMU_VX ");
   return false;
    }if instr &  MASK_VREV8_V ==  MATCH_VREV8_V { 
    
    self.debug_gaspard("VREV8_V ");
   return false;
    }if instr &  MASK_VRGATHER_VI ==  MATCH_VRGATHER_VI { 
    
    self.debug_gaspard("VRGATHER_VI ");
   return false;
    }if instr &  MASK_VRGATHER_VV ==  MATCH_VRGATHER_VV { 
    
    self.debug_gaspard("VRGATHER_VV ");
   return false;
    }if instr &  MASK_VRGATHER_VX ==  MATCH_VRGATHER_VX { 
    
    self.debug_gaspard("VRGATHER_VX ");
   return false;
    }if instr &  MASK_VRGATHEREI16_VV ==  MATCH_VRGATHEREI16_VV { 
    
    self.debug_gaspard("VRGATHEREI16_VV ");
   return false;
    }if instr &  MASK_VROL_VV ==  MATCH_VROL_VV { 
    
    self.debug_gaspard("VROL_VV ");
   return false;
    }if instr &  MASK_VROL_VX ==  MATCH_VROL_VX { 
    
    self.debug_gaspard("VROL_VX ");
   return false;
    }if instr &  MASK_VROR_VI ==  MATCH_VROR_VI { 
    
    self.debug_gaspard("VROR_VI ");
   return false;
    }if instr &  MASK_VROR_VV ==  MATCH_VROR_VV { 
    
    self.debug_gaspard("VROR_VV ");
   return false;
    }if instr &  MASK_VROR_VX ==  MATCH_VROR_VX { 
    
    self.debug_gaspard("VROR_VX ");
   return false;
    }if instr &  MASK_VRSUB_VI ==  MATCH_VRSUB_VI { 
    
    self.debug_gaspard("VRSUB_VI ");
   return false;
    }if instr &  MASK_VRSUB_VX ==  MATCH_VRSUB_VX { 
    
    self.debug_gaspard("VRSUB_VX ");
   return false;
    }if instr &  MASK_VS1R_V ==  MATCH_VS1R_V { 
    
    self.debug_gaspard("VS1R_V ");
   return false;
    }if instr &  MASK_VS2R_V ==  MATCH_VS2R_V { 
    
    self.debug_gaspard("VS2R_V ");
   return false;
    }if instr &  MASK_VS4R_V ==  MATCH_VS4R_V { 
    
    self.debug_gaspard("VS4R_V ");
   return false;
    }if instr &  MASK_VS8R_V ==  MATCH_VS8R_V { 
    
    self.debug_gaspard("VS8R_V ");
   return false;
    }if instr &  MASK_VSADD_VI ==  MATCH_VSADD_VI { 
    
    self.debug_gaspard("VSADD_VI ");
   return false;
    }if instr &  MASK_VSADD_VV ==  MATCH_VSADD_VV { 
    
    self.debug_gaspard("VSADD_VV ");
   return false;
    }if instr &  MASK_VSADD_VX ==  MATCH_VSADD_VX { 
    
    self.debug_gaspard("VSADD_VX ");
   return false;
    }if instr &  MASK_VSADDU_VI ==  MATCH_VSADDU_VI { 
    
    self.debug_gaspard("VSADDU_VI ");
   return false;
    }if instr &  MASK_VSADDU_VV ==  MATCH_VSADDU_VV { 
    
    self.debug_gaspard("VSADDU_VV ");
   return false;
    }if instr &  MASK_VSADDU_VX ==  MATCH_VSADDU_VX { 
    
    self.debug_gaspard("VSADDU_VX ");
   return false;
    }if instr &  MASK_VSBC_VVM ==  MATCH_VSBC_VVM { 
    
    self.debug_gaspard("VSBC_VVM ");
   return false;
    }if instr &  MASK_VSBC_VXM ==  MATCH_VSBC_VXM { 
    
    self.debug_gaspard("VSBC_VXM ");
   return false;
    }if instr &  MASK_VSE1024_V ==  MATCH_VSE1024_V { 
    
    self.debug_gaspard("VSE1024_V ");
   return false;
    }if instr &  MASK_VSE128_V ==  MATCH_VSE128_V { 
    
    self.debug_gaspard("VSE128_V ");
   return false;
    }if instr &  MASK_VSE16_V ==  MATCH_VSE16_V { 
    
    self.debug_gaspard("VSE16_V ");
   return false;
    }if instr &  MASK_VSE1_V ==  MATCH_VSE1_V { 
    
    self.debug_gaspard("VSE1_V ");
   return false;
    }if instr &  MASK_VSE256_V ==  MATCH_VSE256_V { 
    
    self.debug_gaspard("VSE256_V ");
   return false;
    }if instr &  MASK_VSE32_V ==  MATCH_VSE32_V { 
    
    self.debug_gaspard("VSE32_V ");
   return false;
    }if instr &  MASK_VSE512_V ==  MATCH_VSE512_V { 
    
    self.debug_gaspard("VSE512_V ");
   return false;
    }if instr &  MASK_VSE64_V ==  MATCH_VSE64_V { 
    
    self.debug_gaspard("VSE64_V ");
   return false;
    }if instr &  MASK_VSE8_V ==  MATCH_VSE8_V { 
    
    self.debug_gaspard("VSE8_V ");
   return false;
    }if instr &  MASK_VSETIVLI ==  MATCH_VSETIVLI { 
    
    self.debug_gaspard("VSETIVLI ");
   return false;
    }if instr &  MASK_VSETVL ==  MATCH_VSETVL { 
    
    self.debug_gaspard("VSETVL ");
   return false;
    }if instr &  MASK_VSETVLI ==  MATCH_VSETVLI { 
    
    self.debug_gaspard("VSETVLI ");
   return false;
    }if instr &  MASK_VSEXT_VF2 ==  MATCH_VSEXT_VF2 { 
    
    self.debug_gaspard("VSEXT_VF2 ");
   return false;
    }if instr &  MASK_VSEXT_VF4 ==  MATCH_VSEXT_VF4 { 
    
    self.debug_gaspard("VSEXT_VF4 ");
   return false;
    }if instr &  MASK_VSEXT_VF8 ==  MATCH_VSEXT_VF8 { 
    
    self.debug_gaspard("VSEXT_VF8 ");
   return false;
    }if instr &  MASK_VSHA2CH_VV ==  MATCH_VSHA2CH_VV { 
    
    self.debug_gaspard("VSHA2CH_VV ");
   return false;
    }if instr &  MASK_VSHA2CL_VV ==  MATCH_VSHA2CL_VV { 
    
    self.debug_gaspard("VSHA2CL_VV ");
   return false;
    }if instr &  MASK_VSHA2MS_VV ==  MATCH_VSHA2MS_VV { 
    
    self.debug_gaspard("VSHA2MS_VV ");
   return false;
    }if instr &  MASK_VSLIDE1DOWN_VX ==  MATCH_VSLIDE1DOWN_VX { 
    
    self.debug_gaspard("VSLIDE1DOWN_VX ");
   return false;
    }if instr &  MASK_VSLIDE1UP_VX ==  MATCH_VSLIDE1UP_VX { 
    
    self.debug_gaspard("VSLIDE1UP_VX ");
   return false;
    }if instr &  MASK_VSLIDEDOWN_VI ==  MATCH_VSLIDEDOWN_VI { 
    
    self.debug_gaspard("VSLIDEDOWN_VI ");
   return false;
    }if instr &  MASK_VSLIDEDOWN_VX ==  MATCH_VSLIDEDOWN_VX { 
    
    self.debug_gaspard("VSLIDEDOWN_VX ");
   return false;
    }if instr &  MASK_VSLIDEUP_VI ==  MATCH_VSLIDEUP_VI { 
    
    self.debug_gaspard("VSLIDEUP_VI ");
   return false;
    }if instr &  MASK_VSLIDEUP_VX ==  MATCH_VSLIDEUP_VX { 
    
    self.debug_gaspard("VSLIDEUP_VX ");
   return false;
    }if instr &  MASK_VSLL_VI ==  MATCH_VSLL_VI { 
    
    self.debug_gaspard("VSLL_VI ");
   return false;
    }if instr &  MASK_VSLL_VV ==  MATCH_VSLL_VV { 
    
    self.debug_gaspard("VSLL_VV ");
   return false;
    }if instr &  MASK_VSLL_VX ==  MATCH_VSLL_VX { 
    
    self.debug_gaspard("VSLL_VX ");
   return false;
    }if instr &  MASK_VSM3C_VI ==  MATCH_VSM3C_VI { 
    
    self.debug_gaspard("VSM3C_VI ");
   return false;
    }if instr &  MASK_VSM3ME_VV ==  MATCH_VSM3ME_VV { 
    
    self.debug_gaspard("VSM3ME_VV ");
   return false;
    }if instr &  MASK_VSM4K_VI ==  MATCH_VSM4K_VI { 
    
    self.debug_gaspard("VSM4K_VI ");
   return false;
    }if instr &  MASK_VSM4R_VS ==  MATCH_VSM4R_VS { 
    
    self.debug_gaspard("VSM4R_VS ");
   return false;
    }if instr &  MASK_VSM4R_VV ==  MATCH_VSM4R_VV { 
    
    self.debug_gaspard("VSM4R_VV ");
   return false;
    }if instr &  MASK_VSM_V ==  MATCH_VSM_V { 
    
    self.debug_gaspard("VSM_V ");
   return false;
    }if instr &  MASK_VSMUL_VV ==  MATCH_VSMUL_VV { 
    
    self.debug_gaspard("VSMUL_VV ");
   return false;
    }if instr &  MASK_VSMUL_VX ==  MATCH_VSMUL_VX { 
    
    self.debug_gaspard("VSMUL_VX ");
   return false;
    }if instr &  MASK_VSOXEI1024_V ==  MATCH_VSOXEI1024_V { 
    
    self.debug_gaspard("VSOXEI1024_V ");
   return false;
    }if instr &  MASK_VSOXEI128_V ==  MATCH_VSOXEI128_V { 
    
    self.debug_gaspard("VSOXEI128_V ");
   return false;
    }if instr &  MASK_VSOXEI16_V ==  MATCH_VSOXEI16_V { 
    
    self.debug_gaspard("VSOXEI16_V ");
   return false;
    }if instr &  MASK_VSOXEI256_V ==  MATCH_VSOXEI256_V { 
    
    self.debug_gaspard("VSOXEI256_V ");
   return false;
    }if instr &  MASK_VSOXEI32_V ==  MATCH_VSOXEI32_V { 
    
    self.debug_gaspard("VSOXEI32_V ");
   return false;
    }if instr &  MASK_VSOXEI512_V ==  MATCH_VSOXEI512_V { 
    
    self.debug_gaspard("VSOXEI512_V ");
   return false;
    }if instr &  MASK_VSOXEI64_V ==  MATCH_VSOXEI64_V { 
    
    self.debug_gaspard("VSOXEI64_V ");
   return false;
    }if instr &  MASK_VSOXEI8_V ==  MATCH_VSOXEI8_V { 
    
    self.debug_gaspard("VSOXEI8_V ");
   return false;
    }if instr &  MASK_VSRA_VI ==  MATCH_VSRA_VI { 
    
    self.debug_gaspard("VSRA_VI ");
   return false;
    }if instr &  MASK_VSRA_VV ==  MATCH_VSRA_VV { 
    
    self.debug_gaspard("VSRA_VV ");
   return false;
    }if instr &  MASK_VSRA_VX ==  MATCH_VSRA_VX { 
    
    self.debug_gaspard("VSRA_VX ");
   return false;
    }if instr &  MASK_VSRL_VI ==  MATCH_VSRL_VI { 
    
    self.debug_gaspard("VSRL_VI ");
   return false;
    }if instr &  MASK_VSRL_VV ==  MATCH_VSRL_VV { 
    
    self.debug_gaspard("VSRL_VV ");
   return false;
    }if instr &  MASK_VSRL_VX ==  MATCH_VSRL_VX { 
    
    self.debug_gaspard("VSRL_VX ");
   return false;
    }if instr &  MASK_VSSE1024_V ==  MATCH_VSSE1024_V { 
    
    self.debug_gaspard("VSSE1024_V ");
   return false;
    }if instr &  MASK_VSSE128_V ==  MATCH_VSSE128_V { 
    
    self.debug_gaspard("VSSE128_V ");
   return false;
    }if instr &  MASK_VSSE16_V ==  MATCH_VSSE16_V { 
    
    self.debug_gaspard("VSSE16_V ");
   return false;
    }if instr &  MASK_VSSE256_V ==  MATCH_VSSE256_V { 
    
    self.debug_gaspard("VSSE256_V ");
   return false;
    }if instr &  MASK_VSSE32_V ==  MATCH_VSSE32_V { 
    
    self.debug_gaspard("VSSE32_V ");
   return false;
    }if instr &  MASK_VSSE512_V ==  MATCH_VSSE512_V { 
    
    self.debug_gaspard("VSSE512_V ");
   return false;
    }if instr &  MASK_VSSE64_V ==  MATCH_VSSE64_V { 
    
    self.debug_gaspard("VSSE64_V ");
   return false;
    }if instr &  MASK_VSSE8_V ==  MATCH_VSSE8_V { 
    
    self.debug_gaspard("VSSE8_V ");
   return false;
    }if instr &  MASK_VSSRA_VI ==  MATCH_VSSRA_VI { 
    
    self.debug_gaspard("VSSRA_VI ");
   return false;
    }if instr &  MASK_VSSRA_VV ==  MATCH_VSSRA_VV { 
    
    self.debug_gaspard("VSSRA_VV ");
   return false;
    }if instr &  MASK_VSSRA_VX ==  MATCH_VSSRA_VX { 
    
    self.debug_gaspard("VSSRA_VX ");
   return false;
    }if instr &  MASK_VSSRL_VI ==  MATCH_VSSRL_VI { 
    
    self.debug_gaspard("VSSRL_VI ");
   return false;
    }if instr &  MASK_VSSRL_VV ==  MATCH_VSSRL_VV { 
    
    self.debug_gaspard("VSSRL_VV ");
   return false;
    }if instr &  MASK_VSSRL_VX ==  MATCH_VSSRL_VX { 
    
    self.debug_gaspard("VSSRL_VX ");
   return false;
    }if instr &  MASK_VSSUB_VV ==  MATCH_VSSUB_VV { 
    
    self.debug_gaspard("VSSUB_VV ");
   return false;
    }if instr &  MASK_VSSUB_VX ==  MATCH_VSSUB_VX { 
    
    self.debug_gaspard("VSSUB_VX ");
   return false;
    }if instr &  MASK_VSSUBU_VV ==  MATCH_VSSUBU_VV { 
    
    self.debug_gaspard("VSSUBU_VV ");
   return false;
    }if instr &  MASK_VSSUBU_VX ==  MATCH_VSSUBU_VX { 
    
    self.debug_gaspard("VSSUBU_VX ");
   return false;
    }if instr &  MASK_VSUB_VV ==  MATCH_VSUB_VV { 
    
    self.debug_gaspard("VSUB_VV ");
   return false;
    }if instr &  MASK_VSUB_VX ==  MATCH_VSUB_VX { 
    
    self.debug_gaspard("VSUB_VX ");
   return false;
    }if instr &  MASK_VSUXEI1024_V ==  MATCH_VSUXEI1024_V { 
    
    self.debug_gaspard("VSUXEI1024_V ");
   return false;
    }if instr &  MASK_VSUXEI128_V ==  MATCH_VSUXEI128_V { 
    
    self.debug_gaspard("VSUXEI128_V ");
   return false;
    }if instr &  MASK_VSUXEI16_V ==  MATCH_VSUXEI16_V { 
    
    self.debug_gaspard("VSUXEI16_V ");
   return false;
    }if instr &  MASK_VSUXEI256_V ==  MATCH_VSUXEI256_V { 
    
    self.debug_gaspard("VSUXEI256_V ");
   return false;
    }if instr &  MASK_VSUXEI32_V ==  MATCH_VSUXEI32_V { 
    
    self.debug_gaspard("VSUXEI32_V ");
   return false;
    }if instr &  MASK_VSUXEI512_V ==  MATCH_VSUXEI512_V { 
    
    self.debug_gaspard("VSUXEI512_V ");
   return false;
    }if instr &  MASK_VSUXEI64_V ==  MATCH_VSUXEI64_V { 
    
    self.debug_gaspard("VSUXEI64_V ");
   return false;
    }if instr &  MASK_VSUXEI8_V ==  MATCH_VSUXEI8_V { 
    
    self.debug_gaspard("VSUXEI8_V ");
   return false;
    }if instr &  MASK_VWADD_VV ==  MATCH_VWADD_VV { 
    
    self.debug_gaspard("VWADD_VV ");
   return false;
    }if instr &  MASK_VWADD_VX ==  MATCH_VWADD_VX { 
    
    self.debug_gaspard("VWADD_VX ");
   return false;
    }if instr &  MASK_VWADD_WV ==  MATCH_VWADD_WV { 
    
    self.debug_gaspard("VWADD_WV ");
   return false;
    }if instr &  MASK_VWADD_WX ==  MATCH_VWADD_WX { 
    
    self.debug_gaspard("VWADD_WX ");
   return false;
    }if instr &  MASK_VWADDU_VV ==  MATCH_VWADDU_VV { 
    
    self.debug_gaspard("VWADDU_VV ");
   return false;
    }if instr &  MASK_VWADDU_VX ==  MATCH_VWADDU_VX { 
    
    self.debug_gaspard("VWADDU_VX ");
   return false;
    }if instr &  MASK_VWADDU_WV ==  MATCH_VWADDU_WV { 
    
    self.debug_gaspard("VWADDU_WV ");
   return false;
    }if instr &  MASK_VWADDU_WX ==  MATCH_VWADDU_WX { 
    
    self.debug_gaspard("VWADDU_WX ");
   return false;
    }if instr &  MASK_VWMACC_VV ==  MATCH_VWMACC_VV { 
    
    self.debug_gaspard("VWMACC_VV ");
   return false;
    }if instr &  MASK_VWMACC_VX ==  MATCH_VWMACC_VX { 
    
    self.debug_gaspard("VWMACC_VX ");
   return false;
    }if instr &  MASK_VWMACCSU_VV ==  MATCH_VWMACCSU_VV { 
    
    self.debug_gaspard("VWMACCSU_VV ");
   return false;
    }if instr &  MASK_VWMACCSU_VX ==  MATCH_VWMACCSU_VX { 
    
    self.debug_gaspard("VWMACCSU_VX ");
   return false;
    }if instr &  MASK_VWMACCU_VV ==  MATCH_VWMACCU_VV { 
    
    self.debug_gaspard("VWMACCU_VV ");
   return false;
    }if instr &  MASK_VWMACCU_VX ==  MATCH_VWMACCU_VX { 
    
    self.debug_gaspard("VWMACCU_VX ");
   return false;
    }if instr &  MASK_VWMACCUS_VX ==  MATCH_VWMACCUS_VX { 
    
    self.debug_gaspard("VWMACCUS_VX ");
   return false;
    }if instr &  MASK_VWMUL_VV ==  MATCH_VWMUL_VV { 
    
    self.debug_gaspard("VWMUL_VV ");
   return false;
    }if instr &  MASK_VWMUL_VX ==  MATCH_VWMUL_VX { 
    
    self.debug_gaspard("VWMUL_VX ");
   return false;
    }if instr &  MASK_VWMULSU_VV ==  MATCH_VWMULSU_VV { 
    
    self.debug_gaspard("VWMULSU_VV ");
   return false;
    }if instr &  MASK_VWMULSU_VX ==  MATCH_VWMULSU_VX { 
    
    self.debug_gaspard("VWMULSU_VX ");
   return false;
    }if instr &  MASK_VWMULU_VV ==  MATCH_VWMULU_VV { 
    
    self.debug_gaspard("VWMULU_VV ");
   return false;
    }if instr &  MASK_VWMULU_VX ==  MATCH_VWMULU_VX { 
    
    self.debug_gaspard("VWMULU_VX ");
   return false;
    }if instr &  MASK_VWREDSUM_VS ==  MATCH_VWREDSUM_VS { 
    
    self.debug_gaspard("VWREDSUM_VS ");
   return false;
    }if instr &  MASK_VWREDSUMU_VS ==  MATCH_VWREDSUMU_VS { 
    
    self.debug_gaspard("VWREDSUMU_VS ");
   return false;
    }if instr &  MASK_VWSLL_VI ==  MATCH_VWSLL_VI { 
    
    self.debug_gaspard("VWSLL_VI ");
   return false;
    }if instr &  MASK_VWSLL_VV ==  MATCH_VWSLL_VV { 
    
    self.debug_gaspard("VWSLL_VV ");
   return false;
    }if instr &  MASK_VWSLL_VX ==  MATCH_VWSLL_VX { 
    
    self.debug_gaspard("VWSLL_VX ");
   return false;
    }if instr &  MASK_VWSUB_VV ==  MATCH_VWSUB_VV { 
    
    self.debug_gaspard("VWSUB_VV ");
   return false;
    }if instr &  MASK_VWSUB_VX ==  MATCH_VWSUB_VX { 
    
    self.debug_gaspard("VWSUB_VX ");
   return false;
    }if instr &  MASK_VWSUB_WV ==  MATCH_VWSUB_WV { 
    
    self.debug_gaspard("VWSUB_WV ");
   return false;
    }if instr &  MASK_VWSUB_WX ==  MATCH_VWSUB_WX { 
    
    self.debug_gaspard("VWSUB_WX ");
   return false;
    }if instr &  MASK_VWSUBU_VV ==  MATCH_VWSUBU_VV { 
    
    self.debug_gaspard("VWSUBU_VV ");
   return false;
    }if instr &  MASK_VWSUBU_VX ==  MATCH_VWSUBU_VX { 
    
    self.debug_gaspard("VWSUBU_VX ");
   return false;
    }if instr &  MASK_VWSUBU_WV ==  MATCH_VWSUBU_WV { 
    
    self.debug_gaspard("VWSUBU_WV ");
   return false;
    }if instr &  MASK_VWSUBU_WX ==  MATCH_VWSUBU_WX { 
    
    self.debug_gaspard("VWSUBU_WX ");
   return false;
    }if instr &  MASK_VXOR_VI ==  MATCH_VXOR_VI { 
    
    self.debug_gaspard("VXOR_VI ");
   return false;
    }if instr &  MASK_VXOR_VV ==  MATCH_VXOR_VV { 
    
    self.debug_gaspard("VXOR_VV ");
   return false;
    }if instr &  MASK_VXOR_VX ==  MATCH_VXOR_VX { 
    
    self.debug_gaspard("VXOR_VX ");
   return false;
    }if instr &  MASK_VZEXT_VF2 ==  MATCH_VZEXT_VF2 { 
    
    self.debug_gaspard("VZEXT_VF2 ");
   return false;
    }if instr &  MASK_VZEXT_VF4 ==  MATCH_VZEXT_VF4 { 
    
    self.debug_gaspard("VZEXT_VF4 ");
   return false;
    }if instr &  MASK_VZEXT_VF8 ==  MATCH_VZEXT_VF8 { 
    
    self.debug_gaspard("VZEXT_VF8 ");
   return false;
    }if instr &  MASK_WFI ==  MATCH_WFI { 
    
    self.debug_gaspard("WFI ");
   return false;
    }if instr &  MASK_WRS_NTO ==  MATCH_WRS_NTO { 
    
    self.debug_gaspard("WRS_NTO ");
   return false;
    }if instr &  MASK_WRS_STO ==  MATCH_WRS_STO { 
    
    self.debug_gaspard("WRS_STO ");
   return false;
    }if instr &  MASK_XNOR ==  MATCH_XNOR { 
    
    self.debug_gaspard("XNOR ");
   return false;
    }if instr &  MASK_XOR ==  MATCH_XOR { 
    
    self.debug_gaspard("XOR ");



    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let rs2 = instr >> 20 & 0b1111;
    self.clear_flags(); 
 
 
    self.check_data_unsigned(self.regs[rs1 as usize],self.regs[rs2 as usize]);
self.regs[rd as usize] = self.regs[rs1 as usize] ^ self.regs[rs2 as usize];


   return false;
    }if instr &  MASK_XORI ==  MATCH_XORI { 
    
    self.debug_gaspard("XORI ");


    let rd = instr >> 7 & 0b1111;

    let rs1 = instr >> 15  & 0b1111;
    let imm = instr >> 20;
    self.clear_flags(); 


    self.check_data_unsigned(self.regs[rs1 as usize],imm as u64);



    self.regs[rd as usize] = self.regs[rs1 as usize] ^ imm as u64;






   return false;
    }if instr &  MASK_XPERM16 ==  MATCH_XPERM16 { 
    
    self.debug_gaspard("XPERM16 ");
   return false;
    }if instr &  MASK_XPERM32 ==  MATCH_XPERM32 { 
    
    self.debug_gaspard("XPERM32 ");
   return false;
    }if instr &  MASK_XPERM4 ==  MATCH_XPERM4 { 
    
    self.debug_gaspard("XPERM4 ");
   return false;
    }if instr &  MASK_XPERM8 ==  MATCH_XPERM8 { 
    
    self.debug_gaspard("XPERM8 ");
   return false;
    }if instr &  MASK_ZEXT_H ==  MATCH_ZEXT_H { 
    
    self.debug_gaspard("ZEXT_H ");
   return false;
    }if instr &  MASK_ZEXT_H_RV32 ==  MATCH_ZEXT_H_RV32 { 
    
    self.debug_gaspard("ZEXT_H_RV32 ");
   return false;
    }if instr &  MASK_ZIP ==  MATCH_ZIP { 
    
    self.debug_gaspard("ZIP ");
   return false;
    }if instr &  MASK_ZUNPKD810 ==  MATCH_ZUNPKD810 { 
    
    self.debug_gaspard("ZUNPKD810 ");
   return false;
    }if instr &  MASK_ZUNPKD820 ==  MATCH_ZUNPKD820 { 
    
    self.debug_gaspard("ZUNPKD820 ");
   return false;
    }if instr &  MASK_ZUNPKD830 ==  MATCH_ZUNPKD830 { 
    
    self.debug_gaspard("ZUNPKD830 ");
   return false;
    }if instr &  MASK_ZUNPKD831 ==  MATCH_ZUNPKD831 { 
    
    self.debug_gaspard("ZUNPKD831 ");
   return false;
    }if instr &  MASK_ZUNPKD832 ==  MATCH_ZUNPKD832 { 
    
    self.debug_gaspard("ZUNPKD832 ");
   return false;
    }
   
return false;


}

pub fn init_video() {

   // let width = 800 as usize;
 //   let height = 600 as usize;

    // Créer un tableau de pixels (RGBA)
   // self.data_video = vec![0; width * height * 3];


}
fn memory_return_u64(&mut self,adress :u64) -> Option<u64> {




    for (adresse,data) in &self.memory {
    
    if adress == *adresse {

        return Some(*data); 
    
    
    }
    
    
    }
    
return None;
    


}
fn memory_return_u16(&mut self,adress :u64) -> Option<u16> {


// l'adresse existe ??? 




for (adresse,data)  in &self.memory {

if adress == *adresse {

    return Some(*data as u16); 


}


}


return None;

    


}
fn memory_return_u8(&mut self,adress :u64) -> Option<u8> {




// l'adresse existe ??? 




for (adresse,data) in &self.memory {

if adress == *adresse {

    return Some(*data as u8); 


}


}

return None; 


}
fn memory_return_u32(&mut self,adress :u64) -> Option<u32> {



// l'adresse existe ??? 




for (adresse,data) in &self.memory {

if adress == *adresse {

    return Some(*data as u32); 


}


}


return None; 


}

fn draw_framebuffer(&mut self,rouge :u8,vert :u8,bleu :u8 ) {



}
fn video(&mut self,adress :u64,data :u64) {


    // vérifier la video 
    if adress ==0xA0000 {

        // exytrazite les positions

         self.actual_x = (data & 0xffff) as u16;

         self.actual_y =( data>> 16 & 0xffff) as u16;




    } else if adress == 0xA0001 {

        let color = data;

        println!("x et y {:x} {:x} couleur {:x}",self.actual_x,self.actual_y,color);
        let rouge = (color & 0xff) as u8;

        let vert = (color  >> 8 & 0xff) as u8;
        let bleu =( color  >>16  & 0xff) as u8;

        self.draw_framebuffer(rouge,vert,bleu); // rouge vert bleu
    }

}
/// Il y a 1426 instructions riscv
fn run(&mut self) {


  //  init_video();
let index_demarrage = self.find_data_with_addr(self.adress_begin).unwrap(); // fo,nctionne toujours 






/// ici pas de plantage 
let mut upper :u16 = 0; 

let mut half: u16 = 0;

let mut half_bool : bool = false; // si faux alors hALF si true alors upper


self.regs[32] = self.adress_begin;

self.adress_debut_fonction = self.regs[32];
//for i in index_demarrage as usize..self.section_instr_size as usize


println!("commemcent {:x}",self.regs[32]);
while true {




/// ICi, il faudra scanner les 16 bits half et upper
let instruction_tupple  = self.memory_return_u32(self.regs[32]);

self.regs[32] += 4;

// ATTENTION : inscrire la valeur de lancement dans un registre incrementer et y accéder le resigrre est incrémenté par le scan 
if instruction_tupple == None {

    println!("instruction_tupple == None: fin");
    std::process::exit(1);


}

let instruction_32 :u32 = instruction_tupple.unwrap() as u32; 



let two_16bits = self.extract_instr_16(instruction_32);

half = two_16bits.0;
upper = two_16bits.1;









// appeler ici le scan si rvc 

let rvc : bool = self.normal_scan(instruction_32);

if rvc == true {

self.normal_scan(upper as u32);

} 

println!("instr16  {:x} instr 32 : {:x} adress {:x} ",upper,instruction_32,self.regs[32]);

}




}
/// fonction qui permet d'imprimer pour debugger le vecteur contenant le tuple 
fn debug_print(&mut self) {


for (adress,data) in &self.memory {


println!("{:x} {:x} ",adress,data) ;

}



}

pub fn init(&mut self){





self.copy();
//self.debug_print();
self.run();


}
fn sign_extend(&mut self,x: i32, nbits: u32) -> i32 {
	let notherbits = size_of_val(&x) as u32 * 8 - nbits;
  	x.wrapping_shl(notherbits).wrapping_shr(notherbits)
}




}


