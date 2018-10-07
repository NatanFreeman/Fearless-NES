use self::Timing::*;

pub enum Timing {
    Immediate,
    Accumulator,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    ZeroPageRmw,
    ZeroPageXRmw,
    ZeroPageSt,
    ZeroPageXSt,
    ZeroPageYSt,
    Relative,
    Absolute,
    AbsoluteJmp,
    AbsoluteX,
    AbsoluteY,
    AbsoluteRmw,
    AbsoluteXRmw,
    AbsoluteSt,
    AbsoluteXSt,
    AbsoluteYSt,
    AbsoluteYIllegal,
    Indirect,
    IndirectX,
    IndirectY,
    IndirectXSt,
    IndirectYSt,
    IndirectXIllegal,
    IndirectYIllegal,
    Implied,
    Rti,
    Rts,
    Jsr,
    Brk,
    Pha,
    Php,
    Pla,
    Plp,
}

pub static OPCODES: [(Timing, &str); 0x100] = [
    (Brk, ""),
    (IndirectX, "self.ora(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectXIllegal, "self.slo();"),
    (ZeroPage, ""),
    (ZeroPage, "self.ora(val);"),
    (ZeroPageRmw, "self.asl(val);"),
    (ZeroPageRmw, "self.slo();"),
    (Php, ""),
    (Immediate, "self.ora(val);"),
    (Accumulator, "self.asl_a();"),
    (Immediate, "self.anc(val);"),
    (Absolute, ""),
    (Absolute, "self.ora(val);"),
    (AbsoluteRmw, "self.asl(val);"),
    (AbsoluteRmw, "self.slo();"),
    (Relative, "!self.n"),
    (IndirectY, "self.ora(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectYIllegal, "self.slo();"),
    (ZeroPageX, ""),
    (ZeroPageX, "self.ora(val);"),
    (ZeroPageXRmw, "self.asl(val);"),
    (ZeroPageXRmw, "self.slo();"),
    (Implied, "self.c = false;"),
    (AbsoluteY, "self.ora(val);"),
    (Implied, ""),
    (AbsoluteYIllegal, "self.slo();"),
    (AbsoluteX, ""),
    (AbsoluteX, "self.ora(val);"),
    (AbsoluteXRmw, "self.asl(val);"),
    (AbsoluteXRmw, "self.slo();"),
    (Jsr, "self.jsr(val);"),
    (IndirectX, "self.and(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectXIllegal, "self.rla();"),
    (ZeroPage, "self.bit(val);"),
    (ZeroPage, "self.and(val);"),
    (ZeroPageRmw, "self.rol(val);"),
    (ZeroPageRmw, "self.rla();"),
    (Plp, ""),
    (Immediate, "self.and(val);"),
    (Accumulator, "self.rol_a();"),
    (Immediate, "self.anc(val);"),
    (Absolute, "self.bit(val);"),
    (Absolute, "self.and(val);"),
    (AbsoluteRmw, "self.rol(val);"),
    (AbsoluteRmw, "self.rla();"),
    (Relative, "self.n"),
    (IndirectY, "self.and(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectYIllegal, "self.rla();"),
    (ZeroPageX, ""),
    (ZeroPageX, "self.and(val);"),
    (ZeroPageXRmw, "self.rol(val);"),
    (ZeroPageXRmw, "self.rla();"),
    (Implied, "self.c = true;"),
    (AbsoluteY, "self.and(val);"),
    (Implied, ""),
    (AbsoluteYIllegal, "self.rla();"),
    (AbsoluteX, ""),
    (AbsoluteX, "self.and(val);"),
    (AbsoluteXRmw, "self.rol(val);"),
    (AbsoluteXRmw, "self.rla();"),
    (Rti, "self.rti();"),
    (IndirectX, "self.eor(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectXIllegal, "self.sre();"),
    (ZeroPage, ""),
    (ZeroPage, "self.eor(val);"),
    (ZeroPageRmw, "self.lsr(val);"),
    (ZeroPageRmw, "self.sre();"),
    (Pha, ""),
    (Immediate, "self.eor(val);"),
    (Accumulator, "self.lsr_a();"),
    (Immediate, "self.alr(val);"),
    (AbsoluteJmp, ""),
    (Absolute, "self.eor(val);"),
    (AbsoluteRmw, "self.lsr(val);"),
    (AbsoluteRmw, "self.sre();"),
    (Relative, "!self.v"),
    (IndirectY, "self.eor(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectYIllegal, "self.sre();"),
    (ZeroPageX, ""),
    (ZeroPageX, "self.eor(val);"),
    (ZeroPageXRmw, "self.lsr(val);"),
    (ZeroPageXRmw, "self.sre();"),
    (Implied, "self.i = false;"),
    (AbsoluteY, "self.eor(val);"),
    (Implied, ""),
    (AbsoluteYIllegal, "self.sre();"),
    (AbsoluteX, ""),
    (AbsoluteX, "self.eor(val);"),
    (AbsoluteXRmw, "self.lsr(val);"),
    (AbsoluteXRmw, "self.sre();"),
    (Rts, "self.rts();"),
    (IndirectX, "self.adc(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectXIllegal, "self.rra();"),
    (ZeroPage, ""),
    (ZeroPage, "self.adc(val);"),
    (ZeroPageRmw, "self.ror(val);"),
    (ZeroPageRmw, "self.rra();"),
    (Pla, ""),
    (Immediate, "self.adc(val);"),
    (Accumulator, "self.ror_a();"),
    (Immediate, "self.arr(val);"),
    (Indirect, ""),
    (Absolute, "self.adc(val);"),
    (AbsoluteRmw, "self.ror(val);"),
    (AbsoluteRmw, "self.rra();"),
    (Relative, "self.v"),
    (IndirectY, "self.adc(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectYIllegal, "self.rra();"),
    (ZeroPageX, ""),
    (ZeroPageX, "self.adc(val);"),
    (ZeroPageXRmw, "self.ror(val);"),
    (ZeroPageXRmw, "self.rra();"),
    (Implied, "self.i = true;"),
    (AbsoluteY, "self.adc(val);"),
    (Implied, ""),
    (AbsoluteYIllegal, "self.rra();"),
    (AbsoluteX, ""),
    (AbsoluteX, "self.adc(val);"),
    (AbsoluteXRmw, "self.ror(val);"),
    (AbsoluteXRmw, "self.rra();"),
    (Immediate, ""),
    (IndirectXSt, "self.sta();"),
    (Immediate, ""),
    (IndirectX, "self.aax();"),
    (ZeroPageSt, "self.sty();"),
    (ZeroPageSt, "self.sta();"),
    (ZeroPageSt, "self.stx();"),
    (ZeroPage, "self.aax();"),
    (Implied, "self.dey();"),
    (Immediate, ""),
    (Implied, "self.txa();"),
    (Immediate, "self.xaa(val);"),
    (AbsoluteSt, "self.sty();"),
    (AbsoluteSt, "self.sta();"),
    (AbsoluteSt, "self.stx();"),
    (Absolute, "self.aax();"),
    (Relative, "!self.c"),
    (IndirectYSt, "self.sta();"),
    (Immediate, "self.halt = true;"),
    (IndirectYSt, "self.ahx();"),
    (ZeroPageXSt, "self.sty();"),
    (ZeroPageXSt, "self.sta();"),
    (ZeroPageYSt, "self.stx();"),
    (ZeroPageY, "self.aax();"),
    (Implied, "self.tya();"),
    (AbsoluteYSt, "self.sta();"),
    (Implied, "self.txs();"),
    (AbsoluteYSt, "self.tas();"),
    (AbsoluteXSt, "self.shy();"),
    (AbsoluteXSt, "self.sta();"),
    (AbsoluteYSt, "self.shx();"),
    (AbsoluteYSt, "self.ahx();"),
    (Immediate, "self.ldy(val);"),
    (IndirectX, "self.lda(val);"),
    (Immediate, "self.ldx(val);"),
    (IndirectX, "self.lax(val);"),
    (ZeroPage, "self.ldy(val);"),
    (ZeroPage, "self.lda(val);"),
    (ZeroPage, "self.ldx(val);"),
    (ZeroPage, "self.lax(val);"),
    (Implied, "self.tay();"),
    (Immediate, "self.lda(val);"),
    (Implied, "self.tax();"),
    (Immediate, "self.lax(val);"),
    (Absolute, "self.ldy(val);"),
    (Absolute, "self.lda(val);"),
    (Absolute, "self.ldx(val);"),
    (Absolute, "self.lax(val);"),
    (Relative, "self.c"),
    (IndirectY, "self.lda(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectY, "self.lax(val);"),
    (ZeroPageX, "self.ldy(val);"),
    (ZeroPageX, "self.lda(val);"),
    (ZeroPageY, "self.ldx(val);"),
    (ZeroPageY, "self.lax(val);"),
    (Implied, "self.v = false;"),
    (AbsoluteY, "self.lda(val);"),
    (Implied, "self.tsx();"),
    (AbsoluteY, "self.las(val);"),
    (AbsoluteX, "self.ldy(val);"),
    (AbsoluteX, "self.lda(val);"),
    (AbsoluteY, "self.ldx(val);"),
    (AbsoluteY, "self.lax(val);"),
    (Immediate, "self.cpy(val);"),
    (IndirectX, "self.cmp(val);"),
    (Immediate, ""),
    (IndirectXIllegal, "self.dcp();"),
    (ZeroPage, "self.cpy(val);"),
    (ZeroPage, "self.cmp(val);"),
    (ZeroPageRmw, "self.dec(val);"),
    (ZeroPageRmw, "self.dcp();"),
    (Implied, "self.iny();"),
    (Immediate, "self.cmp(val);"),
    (Implied, "self.dex();"),
    (Immediate, "self.axs(val);"),
    (Absolute, "self.cpy(val);"),
    (Absolute, "self.cmp(val);"),
    (AbsoluteRmw, "self.dec(val);"),
    (AbsoluteRmw, "self.dcp();"),
    (Relative, "!self.z"),
    (IndirectY, "self.cmp(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectYIllegal, "self.dcp();"),
    (ZeroPageX, ""),
    (ZeroPageX, "self.cmp(val);"),
    (ZeroPageXRmw, "self.dec(val);"),
    (ZeroPageXRmw, "self.dcp();"),
    (Implied, "self.d = false;"),
    (AbsoluteY, "self.cmp(val);"),
    (Implied, ""),
    (AbsoluteYIllegal, "self.dcp();"),
    (AbsoluteX, ""),
    (AbsoluteX, "self.cmp(val);"),
    (AbsoluteXRmw, "self.dec(val);"),
    (AbsoluteXRmw, "self.dcp();"),
    (Immediate, "self.cpx(val);"),
    (IndirectX, "self.sbc(val);"),
    (Immediate, ""),
    (IndirectXIllegal, "self.isc();"),
    (ZeroPage, "self.cpx(val);"),
    (ZeroPage, "self.sbc(val);"),
    (ZeroPageRmw, "self.inc(val);"),
    (ZeroPageRmw, "self.isc();"),
    (Implied, "self.inx();"),
    (Immediate, "self.sbc(val);"),
    (Implied, ""),
    (Immediate, "self.sbc(val);"),
    (Absolute, "self.cpx(val);"),
    (Absolute, "self.sbc(val);"),
    (AbsoluteRmw, "self.inc(val);"),
    (AbsoluteRmw, "self.isc();"),
    (Relative, "self.z"),
    (IndirectY, "self.sbc(val);"),
    (Immediate, "self.halt = true;"),
    (IndirectYIllegal, "self.isc();"),
    (ZeroPageX, ""),
    (ZeroPageX, "self.sbc(val);"),
    (ZeroPageXRmw, "self.inc(val);"),
    (ZeroPageXRmw, "self.isc();"),
    (Implied, "self.d = true;"),
    (AbsoluteY, "self.sbc(val);"),
    (Implied, ""),
    (AbsoluteYIllegal, "self.isc();"),
    (AbsoluteX, ""),
    (AbsoluteX, "self.sbc(val);"),
    (AbsoluteXRmw, "self.inc(val);"),
    (AbsoluteXRmw, "self.isc();"),
];