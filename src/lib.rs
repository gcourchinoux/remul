pub mod lib {
// TOUT TRANSFORMER EN FONCTION

/*
#[macro_export]
macro_rules! RV_X {
    // `()` indicates that the macro takes no argument.
    ($x:expr,$s:expr,$n:expr) => {
        // The macro will expand into the contents of this block.
      return  ((($x) >> ($s)) & ((1 << ($n)) - 1));
    };
}
*/
pub fn RV_X(x: u32,s : u32, n :u32) -> u32{


  return  (((x) >> (s)) & ((1 << (n)) - 1));

}

/*
#[macro_export]
macro_rules! RV_IMM_SIGN {
    // `()` indicates that the macro takes no argument.
    (x) => {
        // The macro will expand into the contents of this block.
      return  (-(((x) >> 31) & 1));
    };
}
*/
pub fn RV_IMM_SIGN(x: u32) -> u32{


  return  ((((x) >> 31) & 1));

}

/*
#[macro_export]
macro_rules! RV_X_SIGNED {
    // `()` indicates that the macro takes no argument.
    (x,s,n) => {
        // The macro will expand into the contents of this block.
      return  (RV_X(x, s, n) | ((-(RV_X(x, (s + n - 1), 1)))));
    };
}
*/
pub fn RV_X_SIGNED(x: u32,s : u32, n :u32) -> u32{


  return  (RV_X(x, s, n) | (((RV_X(x, (s + n - 1), 1)))));

}



/*
#define EXTRACT_ITYPE_IMM(x) \
  (RV_X(x, 20, 12) | (RV_IMM_SIGN(x) << 12))
*/
/*
#[macro_export]
macro_rules! EXTRACT_ITYPE_IMM {
    // `()` indicates that the macro takes no argument.
    (x) => {
        // The macro will expand into the contents of this block.
      return   (RV_X!(x, 20, 12) | (RV_IMM_SIGN!(x) << 12));
    };
}
*/

pub fn EXTRACT_ITYPE_IMM(x: u32) -> u32{


  return     (RV_X(x, 20, 12) | (RV_IMM_SIGN(x) << 12));

}


/*
#define EXTRACT_STYPE_IMM(x) \
  (RV_X(x, 7, 5) | (RV_X(x, 25, 7) << 5) | (RV_IMM_SIGN(x) << 12))
  */
/*  
#[macro_export]
 macro_rules! EXTRACT_STYPE_IMM {
    // `()` indicates that the macro takes no argument.
    (x) => {
        // The macro will expand into the contents of this block.
      return   (RV_X(x, 7, 5) | (RV_X(x, 25, 7) << 5) | (RV_IMM_SIGN(x) << 12));
    };
}
*/
pub fn EXTRACT_STYPE_IMM(x: u32) -> u32{


  return   (RV_X(x, 7, 5) | (RV_X(x, 25, 7) << 5) | (RV_IMM_SIGN(x) << 12));

}


/*
#define EXTRACT_BTYPE_IMM(x) \
  ((RV_X(x, 8, 4) << 1) | (RV_X(x, 25, 6) << 5) | (RV_X(x, 7, 1) << 11) | (RV_IMM_SIGN(x) << 12))
*/
/*
#[macro_export]

macro_rules! EXTRACT_BTYPE_IMM {
  (x) => {
    return   ((RV_X(x, 8, 4) << 1) | (RV_X(x, 25, 6) << 5) | (RV_X(x, 7, 1) << 11) | (RV_IMM_SIGN(x) << 12));
  };
}
*/
pub fn EXTRACT_BTYPE_IMM(x: u32) -> u32{


  return   ((RV_X(x, 8, 4) << 1) | (RV_X(x, 25, 6) << 5) | (RV_X(x, 7, 1) << 11) | (RV_IMM_SIGN(x) << 12));

}

/*
  #define EXTRACT_UTYPE_IMM(x) \
  ((RV_X(x, 12, 20) << 12) | (RV_IMM_SIGN(x) << 32))
*/
/*
#[macro_export]

macro_rules! EXTRACT_UTYPE_IMM {
    (x) => {
      return   ((RV_X(x, 12, 20) << 12) | (RV_IMM_SIGN(x) << 32));
    };
}
*/

pub fn EXTRACT_UTYPE_IMM(x: u32) -> u32{


  return   x >> 12 & 0b1111111111111111111;

}

/*
#define EXTRACT_JTYPE_IMM(x) \
  ((RV_X(x, 21, 10) << 1) | (RV_X(x, 20, 1) << 11) | (RV_X(x, 12, 8) << 12) | (RV_IMM_SIGN(x) << 20))
*/
/*
#[macro_export]

macro_rules! EXTRACT_JTYPE_IMM {
  (x) => {
    return   ((RV_X(x, 21, 10) << 1) | (RV_X(x, 20, 1) << 11) | (RV_X(x, 12, 8) << 12) | (RV_IMM_SIGN(x) << 20));
  };
}

*/
pub fn EXTRACT_JTYPE_IMM(x: u32) -> u32{


  return   ((RV_X(x, 21, 10) << 1) | (RV_X(x, 20, 1) << 11) | (RV_X(x, 12, 8) << 12) | (RV_IMM_SIGN(x) << 20));

}

/*
  #define EXTRACT_CITYPE_IMM(x) \
  (RV_X(x, 2, 5) | (-RV_X(x, 12, 1) << 5))
*/
/*
#[macro_export]

macro_rules! EXTRACT_CITYPE_IMM {
  (x) => {
    return    (RV_X!(x, 2, 5) | (-RV_X!(x, 12, 1) << 5));
  };
}
*/
pub fn EXTRACT_CITYPE_IMM(x: u32) -> u32{


  return    (RV_X(x, 2, 5) | (RV_X(x, 12, 1) << 5));
}



//#define RISCV_IMM_BITS 12
const  RISCV_IMM_BITS :u32 = 12;

/*
#[macro_export]

macro_rules! RISCV_IMM_BITS {
  (x) => {
    return    (12);
  };
}  
*/

/*
  #define EXTRACT_CITYPE_LUI_IMM(x) \
  (EXTRACT_CITYPE_IMM (x) << RISCV_IMM_BITS)
*/
/*
#[macro_export]

macro_rules! EXTRACT_CITYPE_LUI_IMM {
    (x) => {
      return    (EXTRACT_CITYPE_IMM! (x) << RISCV_IMM_BITS!());
    };
}  
*/
pub fn EXTRACT_CITYPE_LUI_IMM(x: u32) -> u32{


  return    (EXTRACT_CITYPE_IMM (x) << RISCV_IMM_BITS);
}


/*
#define EXTRACT_CITYPE_ADDI16SP_IMM(x) \
  ((RV_X(x, 6, 1) << 4) | (RV_X(x, 2, 1) << 5) | (RV_X(x, 5, 1) << 6) | (RV_X(x, 3, 2) << 7) | (-RV_X(x, 12, 1) << 9))
*/
/*
#[macro_export]
macro_rules! EXTRACT_CITYPE_ADDI16SP_IMM {
  ($x:expr) => {
    return    ((RV_X!($x, 6, 1) << 4) | (RV_X!($x, 2, 1) << 5) | (RV_X!($x, 5, 1) << 6) | (RV_X!($x, 3, 2) << 7) | (RV_X!($x, 12, 1) << 9));
  };
}  
*/
pub fn EXTRACT_CITYPE_ADDI16SP_IMM(x: u32) -> u32{


  return    ((RV_X(x, 6, 1) << 4) | (RV_X(x, 2, 1) << 5) | (RV_X(x, 5, 1) << 6) | (RV_X(x, 3, 2) << 7) | (RV_X(x, 12, 1) << 9));
}
/*
  #define EXTRACT_CITYPE_LWSP_IMM(x) \
  ((RV_X(x, 4, 3) << 2) | (RV_X(x, 12, 1) << 5) | (RV_X(x, 2, 2) << 6))
*/
/*
macro_rules! EXTRACT_CITYPE_LWSP_IMM {
  (x) => {
    return    ((RV_X!(x, 4, 3) << 2) | (RV_X!(x, 12, 1) << 5) | (RV_X!(x, 2, 2) << 6));
  };
}  

*/
pub fn EXTRACT_CITYPE_LWSP_IMM(x: u32) -> u32{


  return    ((RV_X(x, 4, 3) << 2) | (RV_X(x, 12, 1) << 5) | (RV_X(x, 2, 2) << 6));
}
/*

  #define EXTRACT_CITYPE_LDSP_IMM(x) \
  ((RV_X(x, 5, 2) << 3) | (RV_X(x, 12, 1) << 5) | (RV_X(x, 2, 3) << 6))
*/
/*
macro_rules! EXTRACT_CITYPE_LDSP_IMM {
  (x) => {
    return     ((RV_X!(x, 5, 2) << 3) | (RV_X!(x, 12, 1) << 5) | (RV_X!(x, 2, 3) << 6));
  };
}  
*/
pub fn EXTRACT_CITYPE_LDSP_IMM(x: u32) -> u32{


  return     ((RV_X(x, 5, 2) << 3) | (RV_X(x, 12, 1) << 5) | (RV_X(x, 2, 3) << 6));
}





/*
  #define EXTRACT_CSSTYPE_IMM(x) \
  (RV_X(x, 7, 6) << 0)
*/
/*
macro_rules! EXTRACT_CSSTYPE_IMM {
    (x) => {
      return      (RV_X!(x, 7, 6) << 0);
    };
}  
*/
pub fn EXTRACT_CSSTYPE_IMM(x: u32) -> u32{


  return      (RV_X(x, 7, 6) << 0);
}


/*
#define EXTRACT_CSSTYPE_SWSP_IMM(x) \
  ((RV_X(x, 9, 4) << 2) | (RV_X(x, 7, 2) << 6))

*/
/*
macro_rules! EXTRACT_CSSTYPE_SWSP_IMM {
  (x) => {
    return ((RV_X!(x, 9, 4) << 2) | (RV_X!(x, 7, 2) << 6));
  };
}  
*/
pub fn EXTRACT_CSSTYPE_SWSP_IMM(x: u32) -> u32{


  return ((RV_X(x, 9, 4) << 2) | (RV_X(x, 7, 2) << 6));
}

/*
#define EXTRACT_CSSTYPE_SDSP_IMM(x) \
  ((RV_X(x, 10, 3) << 3) | (RV_X(x, 7, 3) << 6))

*/
/*
macro_rules! EXTRACT_CSSTYPE_SDSP_IMM {
  (x) => {
    return ((RV_X!(x, 10, 3) << 3) | (RV_X!(x, 7, 3) << 6));
  };
}  
*/
pub fn EXTRACT_CSSTYPE_SDSP_IMM(x: u32) -> u32{


  return ((RV_X(x, 10, 3) << 3) | (RV_X(x, 7, 3) << 6));
}

/*
#define EXTRACT_CIWTYPE_IMM(x) \
  (RV_X(x, 5, 8))
*/
/*
macro_rules! EXTRACT_CIWTYPE_IMM {
  (x) => {
    return   (RV_X!(x, 5, 8));
  };
}  
*/

pub fn EXTRACT_CIWTYPE_IMM(x: u32) -> u32{


  return   (RV_X(x, 5, 8));
}



/*
  #define EXTRACT_CIWTYPE_ADDI4SPN_IMM(x) \
  ((RV_X(x, 6, 1) << 2) | (RV_X(x, 5, 1) << 3) | (RV_X(x, 11, 2) << 4) | (RV_X(x, 7, 4) << 6))
*/
/*
macro_rules! EXTRACT_CIWTYPE_ADDI4SPN_IMM {
    (x) => {
      return   ((RV_X!(x, 6, 1) << 2) | (RV_X!(x, 5, 1) << 3) | (RV_X!(x, 11, 2) << 4) | (RV_X!(x, 7, 4) << 6));
  };
} 
*/

pub fn EXTRACT_CIWTYPE_ADDI4SPN_IMM(x: u32) -> u32{


  return   ((RV_X(x, 6, 1) << 2) | (RV_X(x, 5, 1) << 3) | (RV_X(x, 11, 2) << 4) | (RV_X(x, 7, 4) << 6));
}



/* 
  #define EXTRACT_CLTYPE_IMM(x) \
  ((RV_X(x, 5, 2) << 0) | (RV_X(x, 10, 3) << 2))
*/
/*
  macro_rules! EXTRACT_CLTYPE_IMM {
    (x) => {
      return   ((RV_X!(x, 5, 2) << 0) | (RV_X!(x, 10, 3) << 2));
  };
}  
*/

pub fn EXTRACT_CLTYPE_IMM(x: u32) -> u32{


  return   ((RV_X(x, 5, 2) << 0) | (RV_X(x, 10, 3) << 2));
}


/*
#define EXTRACT_CLTYPE_LW_IMM(x) \
  ((RV_X(x, 6, 1) << 2) | (RV_X(x, 10, 3) << 3) | (RV_X(x, 5, 1) << 6))
*/
/*
macro_rules! EXTRACT_CLTYPE_LW_IMM {
  (x) => {
    return    ((RV_X!(x, 6, 1) << 2) | (RV_X!(x, 10, 3) << 3) | (RV_X!(x, 5, 1) << 6));
};
}

*/
pub fn EXTRACT_CLTYPE_LW_IMM(x: u32) -> u32{


  return    ((RV_X(x, 6, 1) << 2) | (RV_X(x, 10, 3) << 3) | (RV_X(x, 5, 1) << 6));
}


/* 
  #define EXTRACT_CLTYPE_LD_IMM(x) \
  ((RV_X(x, 10, 3) << 3) | (RV_X(x, 5, 2) << 6))
*/
/*
macro_rules! EXTRACT_CLTYPE_LD_IMM {
  (x) => {
    return    ((RV_X!(x, 10, 3) << 3) | (RV_X!(x, 5, 2) << 6));
};
} 
*/
pub fn EXTRACT_CLTYPE_LD_IMM(x: u32) -> u32{


  return    ((RV_X(x, 10, 3) << 3) | (RV_X(x, 5, 2) << 6));
}




/*
#define EXTRACT_CBTYPE_IMM(x) \
  ((RV_X(x, 3, 2) << 1) | (RV_X(x, 10, 2) << 3) | (RV_X(x, 2, 1) << 5) | (RV_X(x, 5, 2) << 6) | (-RV_X(x, 12, 1) << 8))
*/
/*
macro_rules! EXTRACT_CBTYPE_IMM {
  (x) => {
    return   ((RV_X!(x, 3, 2) << 1) | (RV_X!(x, 10, 2) << 3) | (RV_X!(x, 2, 1) << 5) | (RV_X!(x, 5, 2) << 6) | (-RV_X!(x, 12, 1) << 8));
};
}
*/
pub fn EXTRACT_CBTYPE_IMM(x: u32) -> u32{


  return   ((RV_X(x, 3, 2) << 1) | (RV_X(x, 10, 2) << 3) | (RV_X(x, 2, 1) << 5) | (RV_X(x, 5, 2) << 6) | (RV_X(x, 12, 1) << 8));
}

/*
  #define EXTRACT_CJTYPE_IMM(x) \
  ((RV_X(x, 3, 3) << 1) | (RV_X(x, 11, 1) << 4) | (RV_X(x, 2, 1) << 5) | (RV_X(x, 7, 1) << 6) | (RV_X(x, 6, 1) << 7) | (RV_X(x, 9, 2) << 8) | (RV_X(x, 8, 1) << 10) | (-RV_X(x, 12, 1) << 11))
*/
/*
macro_rules! EXTRACT_CJTYPE_IMM {
    (x) => {
      return   ((RV_X!(x, 3, 3) << 1) | (RV_X!(x, 11, 1) << 4) | (RV_X!(x, 2, 1) << 5) | (RV_X!(x, 7, 1) << 6) | (RV_X!(x, 6, 1) << 7) | (RV_X!(x, 9, 2) << 8) | (RV_X!(x, 8, 1) << 10) | (-RV_X!(x, 12, 1) << 11));
  };
}

*/
pub fn EXTRACT_CJTYPE_IMM(x: u32) -> u32{


  return   ((RV_X(x, 3, 3) << 1) | (RV_X(x, 11, 1) << 4) | (RV_X(x, 2, 1) << 5) | (RV_X(x, 7, 1) << 6) | (RV_X(x, 6, 1) << 7) | (RV_X(x, 9, 2) << 8) | (RV_X(x, 8, 1) << 10) | (RV_X(x, 12, 1) << 11));
}

/*
  #define EXTRACT_RVV_VI_IMM(x) \
  (RV_X(x, 15, 5) | (-RV_X(x, 19, 1) << 5))
*/
/*
macro_rules! EXTRACT_RVV_VI_IMM {
  (x) => {
    return   (RV_X!(x, 15, 5) | (-RV_X!(x, 19, 1) << 5));
};
}
*/
pub fn EXTRACT_RVV_VI_IMM(x: u32) -> u32{


  return   (RV_X(x, 15, 5) | (RV_X(x, 19, 1) << 5));
}

/*
#define EXTRACT_RVV_VI_UIMM(x) \
  (RV_X(x, 15, 5))
*/
/*
macro_rules! EXTRACT_RVV_VI_UIMM {
    (x) => {
      return   (RV_X!(x, 15, 5));
  };
}
*/

pub fn EXTRACT_RVV_VI_UIMM(x: u32) -> u32{


  return   (RV_X(x, 15, 5));
}


/*
#define EXTRACT_RVV_VI_UIMM6(x) \
  (RV_X(x, 15, 5) | (RV_X(x, 26, 1) << 5))
 */
 /* 
  macro_rules! EXTRACT_RVV_VI_UIMM6 {
    (x) => {
      return   (RV_X!(x, 15, 5) | (RV_X!(x, 26, 1) << 5));
  };
}
*/
pub fn EXTRACT_RVV_VI_UIMM6(x: u32) -> u32{


  return   (RV_X(x, 15, 5) | (RV_X(x, 26, 1) << 5));
}


/*
#define EXTRACT_RVV_OFFSET(x) \
  (RV_X(x, 29, 3))
*/
/*
  macro_rules! EXTRACT_RVV_OFFSET {
    (x) => {
      return   (RV_X!(x, 29, 3));
  };
}
*/
pub fn EXTRACT_RVV_OFFSET(x: u32) -> u32{


  return   (RV_X(x, 29, 3));
}

/*
#define EXTRACT_RVV_VB_IMM(x) \
  (RV_X(x, 20, 10))

 */
 /* 
macro_rules! EXTRACT_RVV_VB_IMM {
    (x) => {
      return  (RV_X!(x, 20, 10));
  };
}
*/
pub fn EXTRACT_RVV_VB_IMM(x: u32) -> u32{


  return  (RV_X(x, 20, 10));
}

/*
#define EXTRACT_RVV_VC_IMM(x) \
  (RV_X(x, 20, 11))
*/
/*  
macro_rules! EXTRACT_RVV_VC_IMM {
    (x) => {
      return   (RV_X!(x, 20, 11));
  };
}

*/
pub fn EXTRACT_RVV_VC_IMM(x: u32) -> u32{


  return   (RV_X(x, 20, 11));
}
/*
#define EXTRACT_ZCB_BYTE_UIMM(x) \
  (RV_X(x, 6, 1) | (RV_X(x, 5, 1) << 1))
*/
/*
  macro_rules! EXTRACT_ZCB_BYTE_UIMM {
    (x) => {
      return   (RV_X!(x, 6, 1) | (RV_X!(x, 5, 1) << 1));
  };
}
*/

pub fn EXTRACT_ZCB_BYTE_UIMM(x: u32) -> u32{


  return   (RV_X(x, 6, 1) | (RV_X(x, 5, 1) << 1));
}

/*
#define EXTRACT_ZCB_HALFWORD_UIMM(x) \
  (RV_X(x, 5, 1) << 1)
*/
/*
  macro_rules! EXTRACT_ZCB_HALFWORD_UIMM {
    (x) => {
      return   (RV_X!(x, 5, 1) << 1);
  };
}  
*/

pub fn EXTRACT_ZCB_HALFWORD_UIMM(x: u32) -> u32{


  return   (RV_X(x, 5, 1) << 1);
}


/*
/* Vendor-specific (CORE-V) extract macros.  */
#define EXTRACT_CV_IS2_UIMM5(x) \
  (RV_X(x, 20, 5))
*/
/*
  macro_rules! EXTRACT_CV_IS2_UIMM5 {
    (x) => {
      return  (RV_X!(x, 20, 5));
  };
}  
*/
pub fn EXTRACT_CV_IS2_UIMM5(x: u32) -> u32{


  return  (RV_X(x, 20, 5));
}
/*
#define EXTRACT_CV_IS3_UIMM5(x) \
  (RV_X(x, 25, 5)) */
/*
  macro_rules! EXTRACT_CV_IS3_UIMM5 {
    (x) => {
      return  (RV_X!(x, 25, 5)) ;
  };
}  

*/
pub fn EXTRACT_CV_IS3_UIMM5(x: u32) -> u32{


  return  (RV_X(x, 25, 5)) ;
}

}