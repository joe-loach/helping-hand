pub(crate) struct ADC;
impl ADC {
    const ENC: &'static [&'static str] = &[
        "ADC{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "ADC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "ADC{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "ADC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct ADCS;
impl ADCS {
    const ENC: &'static [&'static str] = &[
        "ADCS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "ADCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "ADCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "ADCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct ADD;
impl ADD {
    const ENC: &'static [&'static str] = &[
        "ADD{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "ADD{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "ADD{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "ADD{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
        "ADD{<c>}{<q>} {<Rd>,} SP, #<const>",
        "ADD{<c>}{<q>} {<Rd>,} SP, <Rm> , RRX",
        "ADD{<c>}{<q>} {<Rd>,} SP, <Rm> {, <shift> #<amount>}",
    ];
}
pub(crate) struct ADDS;
impl ADDS {
    const ENC: &'static [&'static str] = &[
        "ADDS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "ADDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "ADDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "ADDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
        "ADDS{<c>}{<q>} {<Rd>,} SP, #<const>",
        "ADDS{<c>}{<q>} {<Rd>,} SP, <Rm> , RRX",
        "ADDS{<c>}{<q>} {<Rd>,} SP, <Rm> {, <shift> #<amount>}",
    ];
}
pub(crate) struct ADR;
impl ADR {
    const ENC: &'static [&'static str] = &[
        "ADD{<c>}{<q>} <Rd>, PC, #<const>",
        "ADR{<c>}{<q>} <Rd>, <label>",
    ];
}
pub(crate) struct AND;
impl AND {
    const ENC: &'static [&'static str] = &[
        "AND{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "AND{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "AND{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "AND{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct ANDS;
impl ANDS {
    const ENC: &'static [&'static str] = &[
        "ANDS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "ANDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "ANDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "ANDS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct B;
impl B {
    const ENC: &'static [&'static str] = &["B{<c>}{<q>} <label>"];
}
pub(crate) struct BFC;
impl BFC {
    const ENC: &'static [&'static str] = &["BFC{<c>}{<q>} <Rd>, #<lsb>, #<width>"];
}
pub(crate) struct BFI;
impl BFI {
    const ENC: &'static [&'static str] = &["BFI{<c>}{<q>} <Rd>, <Rn>, #<lsb>, #<width>"];
}
pub(crate) struct BIC;
impl BIC {
    const ENC: &'static [&'static str] = &[
        "BIC{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "BIC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "BIC{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "BIC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct BICS;
impl BICS {
    const ENC: &'static [&'static str] = &[
        "BICS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "BICS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "BICS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "BICS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct BKPT;
impl BKPT {
    const ENC: &'static [&'static str] = &["BKPT{<q>} {#}<imm>"];
}
pub(crate) struct BL;
impl BL {
    const ENC: &'static [&'static str] = &["BL{<c>}{<q>} <label>"];
}
pub(crate) struct BLX;
impl BLX {
    const ENC: &'static [&'static str] = &["BLX{<c>}{<q>} <Rm>"];
}
pub(crate) struct BX;
impl BX {
    const ENC: &'static [&'static str] = &["BX{<c>}{<q>} <Rm>"];
}
pub(crate) struct BXJ;
impl BXJ {
    const ENC: &'static [&'static str] = &["BXJ{<c>}{<q>} <Rm>"];
}
pub(crate) struct CLREX;
impl CLREX {
    const ENC: &'static [&'static str] = &["CLREX{<c>}{<q>}"];
}
pub(crate) struct CLZ;
impl CLZ {
    const ENC: &'static [&'static str] = &["CLZ{<c>}{<q>} <Rd>, <Rm>"];
}
pub(crate) struct CMN;
impl CMN {
    const ENC: &'static [&'static str] = &[
        "CMN{<c>}{<q>} <Rn>, #<const>",
        "CMN{<c>}{<q>} <Rn>, <Rm>, RRX",
        "CMN{<c>}{<q>} <Rn>, <Rm> {, <shift> #<amount>}",
        "CMN{<c>}{<q>} <Rn>, <Rm>, <type> <Rs>",
    ];
}
pub(crate) struct CMP;
impl CMP {
    const ENC: &'static [&'static str] = &[
        "CMP{<c>}{<q>} <Rn>, #<const>",
        "CMP{<c>}{<q>} <Rn>, <Rm>, RRX",
        "CMP{<c>}{<q>} <Rn>, <Rm> {, <shift> #<amount>}",
        "CMP{<c>}{<q>} <Rn>, <Rm>, <type> <Rs>",
    ];
}
pub(crate) struct CPS;
impl CPS {
    const ENC: &'static [&'static str] = &["CPS{<q>} #<mode>"];
}
pub(crate) struct CPSID;
impl CPSID {
    const ENC: &'static [&'static str] = &["CPSID{<q>} <iflags>", "CPSID{<q>} <iflags> , #<mode>"];
}
pub(crate) struct CPSIE;
impl CPSIE {
    const ENC: &'static [&'static str] = &["CPSIE{<q>} <iflags>", "CPSIE{<q>} <iflags> , #<mode>"];
}
pub(crate) struct CRC32B;
impl CRC32B {
    const ENC: &'static [&'static str] = &["CRC32B{<q>} <Rd>, <Rn>, <Rm>"];
}
pub(crate) struct CRC32CB;
impl CRC32CB {
    const ENC: &'static [&'static str] = &["CRC32CB{<q>} <Rd>, <Rn>, <Rm>"];
}
pub(crate) struct CRC32CH;
impl CRC32CH {
    const ENC: &'static [&'static str] = &["CRC32CH{<q>} <Rd>, <Rn>, <Rm>"];
}
pub(crate) struct CRC32CW;
impl CRC32CW {
    const ENC: &'static [&'static str] = &["CRC32CW{<q>} <Rd>, <Rn>, <Rm>"];
}
pub(crate) struct CRC32H;
impl CRC32H {
    const ENC: &'static [&'static str] = &["CRC32H{<q>} <Rd>, <Rn>, <Rm>"];
}
pub(crate) struct CRC32W;
impl CRC32W {
    const ENC: &'static [&'static str] = &["CRC32W{<q>} <Rd>, <Rn>, <Rm>"];
}
pub(crate) struct CSDB;
impl CSDB {
    const ENC: &'static [&'static str] = &["CSDB{<c>}{<q>}"];
}
pub(crate) struct DBG;
impl DBG {
    const ENC: &'static [&'static str] = &["DBG{<c>}{<q>} #<option>"];
}
pub(crate) struct DMB;
impl DMB {
    const ENC: &'static [&'static str] = &["DMB{<c>}{<q>} {<option>}"];
}
pub(crate) struct DSB;
impl DSB {
    const ENC: &'static [&'static str] = &["DSB{<c>}{<q>} {<option>}"];
}
pub(crate) struct EOR;
impl EOR {
    const ENC: &'static [&'static str] = &[
        "EOR{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "EOR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "EOR{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "EOR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct EORS;
impl EORS {
    const ENC: &'static [&'static str] = &[
        "EORS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "EORS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "EORS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "EORS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct ERET;
impl ERET {
    const ENC: &'static [&'static str] = &["ERET{<c>}{<q>}"];
}
pub(crate) struct ESB;
impl ESB {
    const ENC: &'static [&'static str] = &["ESB{<c>}{<q>}"];
}
pub(crate) struct HLT;
impl HLT {
    const ENC: &'static [&'static str] = &["HLT{<q>} {#}<imm>"];
}
pub(crate) struct HVC;
impl HVC {
    const ENC: &'static [&'static str] = &["HVC{<q>} {#}<imm16>"];
}
pub(crate) struct ISB;
impl ISB {
    const ENC: &'static [&'static str] = &["ISB{<c>}{<q>} {<option>}"];
}
pub(crate) struct LDA;
impl LDA {
    const ENC: &'static [&'static str] = &["LDA{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct LDAB;
impl LDAB {
    const ENC: &'static [&'static str] = &["LDAB{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct LDAEX;
impl LDAEX {
    const ENC: &'static [&'static str] = &["LDAEX{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct LDAEXB;
impl LDAEXB {
    const ENC: &'static [&'static str] = &["LDAEXB{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct LDAEXD;
impl LDAEXD {
    const ENC: &'static [&'static str] = &["LDAEXD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>]"];
}
pub(crate) struct LDAEXH;
impl LDAEXH {
    const ENC: &'static [&'static str] = &["LDAEXH{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct LDAH;
impl LDAH {
    const ENC: &'static [&'static str] = &["LDAH{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct LDC;
impl LDC {
    const ENC: &'static [&'static str] = &[
        "LDC{<c>}{<q>} p14, c5, [<Rn>{, #{+/-}<imm>}]",
        "LDC{<c>}{<q>} p14, c5, [<Rn>], #{+/-}<imm>",
        "LDC{<c>}{<q>} p14, c5, [<Rn>, #{+/-}<imm>]!",
        "LDC{<c>}{<q>} p14, c5, [<Rn>], <option>",
        "LDC{<c>}{<q>} p14, c5, <label>",
    ];
}
pub(crate) struct LDM;
impl LDM {
    const ENC: &'static [&'static str] = &[
        "LDM{IA}{<c>}{<q>} <Rn>{!}, <registers>",
        "LDMDA{<c>}{<q>} <Rn>{!}, <registers>",
        "LDMDB{<c>}{<q>} <Rn>{!}, <registers>",
        "LDMIB{<c>}{<q>} <Rn>{!}, <registers>",
        "LDM{<amode>}{<c>}{<q>} <Rn>{!}, <registers_with_pc>^",
        "LDM{<amode>}{<c>}{<q>} <Rn>, <registers_without_pc>^",
        "POP{<c>}{<q>} <registers>",
    ];
}
pub(crate) struct LDR;
impl LDR {
    const ENC: &'static [&'static str] = &[
        "LDR{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]",
        "LDR{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>",
        "LDR{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!",
        "LDR{<c>}{<q>} <Rt>, <label>",
        "LDR{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]",
        "LDR{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}",
        "LDR{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]!",
        "POP{<c>}{<q>} <single_register_list>",
    ];
}
pub(crate) struct LDRB;
impl LDRB {
    const ENC: &'static [&'static str] = &[
        "LDRB{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]",
        "LDRB{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>",
        "LDRB{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!",
        "LDRB{<c>}{<q>} <Rt>, <label>",
        "LDRB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]",
        "LDRB{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}",
        "LDRB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]!",
    ];
}
pub(crate) struct LDRBT;
impl LDRBT {
    const ENC: &'static [&'static str] = &["LDRBT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}"];
}
pub(crate) struct LDRD;
impl LDRD {
    const ENC: &'static [&'static str] = &[
        "LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn> {, #{+/-}<imm>}]",
        "LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>], #{+/-}<imm>",
        "LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, #{+/-}<imm>]!",
        "LDRD{<c>}{<q>} <Rt>, <Rt2>, <label>",
        "LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, {+/-}<Rm>]",
        "LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>], {+/-}<Rm>",
        "LDRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, {+/-}<Rm>]!",
    ];
}
pub(crate) struct LDREX;
impl LDREX {
    const ENC: &'static [&'static str] = &["LDREX{<c>}{<q>} <Rt>, [<Rn> {, {#}<imm>}]"];
}
pub(crate) struct LDREXB;
impl LDREXB {
    const ENC: &'static [&'static str] = &["LDREXB{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct LDREXD;
impl LDREXD {
    const ENC: &'static [&'static str] = &["LDREXD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>]"];
}
pub(crate) struct LDREXH;
impl LDREXH {
    const ENC: &'static [&'static str] = &["LDREXH{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct LDRH;
impl LDRH {
    const ENC: &'static [&'static str] = &[
        "LDRH{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]",
        "LDRH{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>",
        "LDRH{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!",
        "LDRH{<c>}{<q>} <Rt>, <label>",
        "LDRH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]",
        "LDRH{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>",
        "LDRH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]!",
    ];
}
pub(crate) struct LDRHT;
impl LDRHT {
    const ENC: &'static [&'static str] = &["LDRHT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}"];
}
pub(crate) struct LDRSB;
impl LDRSB {
    const ENC: &'static [&'static str] = &[
        "LDRSB{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]",
        "LDRSB{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>",
        "LDRSB{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!",
        "LDRSB{<c>}{<q>} <Rt>, <label>",
        "LDRSB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]",
        "LDRSB{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>",
        "LDRSB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]!",
    ];
}
pub(crate) struct LDRSBT;
impl LDRSBT {
    const ENC: &'static [&'static str] = &["LDRSBT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}"];
}
pub(crate) struct LDRSH;
impl LDRSH {
    const ENC: &'static [&'static str] = &[
        "LDRSH{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]",
        "LDRSH{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>",
        "LDRSH{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!",
        "LDRSH{<c>}{<q>} <Rt>, <label>",
        "LDRSH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]",
        "LDRSH{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>",
        "LDRSH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]!",
    ];
}
pub(crate) struct LDRSHT;
impl LDRSHT {
    const ENC: &'static [&'static str] = &["LDRSHT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}"];
}
pub(crate) struct LDRT;
impl LDRT {
    const ENC: &'static [&'static str] = &["LDRT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}"];
}
pub(crate) struct MCR;
impl MCR {
    const ENC: &'static [&'static str] =
        &["MCR{<c>}{<q>} <coproc>, {#}<opc1>, <Rt>, <CRn>, <CRm>{, {#}<opc2>}"];
}
pub(crate) struct MCRR;
impl MCRR {
    const ENC: &'static [&'static str] =
        &["MCRR{<c>}{<q>} <coproc>, {#}<opc1>, <Rt>, <Rt2>, <CRm>"];
}
pub(crate) struct MLA;
impl MLA {
    const ENC: &'static [&'static str] = &["MLA{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct MLAS;
impl MLAS {
    const ENC: &'static [&'static str] = &["MLAS{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct MLS;
impl MLS {
    const ENC: &'static [&'static str] = &["MLS{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct MOV;
impl MOV {
    const ENC: &'static [&'static str] = &[
        "ASR{<c>}{<q>} {<Rd>,} <Rm>, #<imm>",
        "ASR{<c>}{<q>} {<Rd>,} <Rm>, <Rs>",
        "LSL{<c>}{<q>} {<Rd>,} <Rm>, #<imm>",
        "LSL{<c>}{<q>} {<Rd>,} <Rm>, <Rs>",
        "LSR{<c>}{<q>} {<Rd>,} <Rm>, #<imm>",
        "LSR{<c>}{<q>} {<Rd>,} <Rm>, <Rs>",
        "MOV{<c>}{<q>} <Rd>, #<const>",
        "MOV{<c>}{<q>} <Rd>, <Rm>, RRX",
        "MOV{<c>}{<q>} <Rd>, <Rm> {, <shift> #<amount>}",
        "MOV{<c>}{<q>} <Rd>, <Rm>, <shift> <Rs>",
        "ROR{<c>}{<q>} {<Rd>,} <Rm>, #<imm>",
        "ROR{<c>}{<q>} {<Rd>,} <Rm>, <Rs>",
        "RRX{<c>}{<q>} {<Rd>,} <Rm>",
    ];
}
pub(crate) struct MOVS;
impl MOVS {
    const ENC: &'static [&'static str] = &[
        "ASRS{<c>}{<q>} {<Rd>,} <Rm>, #<imm>",
        "ASRS{<c>}{<q>} {<Rd>,} <Rm>, <Rs>",
        "LSLS{<c>}{<q>} {<Rd>,} <Rm>, #<imm>",
        "LSLS{<c>}{<q>} {<Rd>,} <Rm>, <Rs>",
        "LSRS{<c>}{<q>} {<Rd>,} <Rm>, #<imm>",
        "LSRS{<c>}{<q>} {<Rd>,} <Rm>, <Rs>",
        "MOVS{<c>}{<q>} <Rd>, #<const>",
        "MOVS{<c>}{<q>} <Rd>, <Rm>, RRX",
        "MOVS{<c>}{<q>} <Rd>, <Rm> {, <shift> #<amount>}",
        "MOVS{<c>}{<q>} <Rd>, <Rm>, <shift> <Rs>",
        "RORS{<c>}{<q>} {<Rd>,} <Rm>, #<imm>",
        "RORS{<c>}{<q>} {<Rd>,} <Rm>, <Rs>",
        "RRXS{<c>}{<q>} {<Rd>,} <Rm>",
    ];
}
pub(crate) struct MOVT;
impl MOVT {
    const ENC: &'static [&'static str] = &["MOVT{<c>}{<q>} <Rd>, #<imm16>"];
}
pub(crate) struct MRC;
impl MRC {
    const ENC: &'static [&'static str] =
        &["MRC{<c>}{<q>} <coproc>, {#}<opc1>, <Rt>, <CRn>, <CRm>{, {#}<opc2>}"];
}
pub(crate) struct MRRC;
impl MRRC {
    const ENC: &'static [&'static str] =
        &["MRRC{<c>}{<q>} <coproc>, {#}<opc1>, <Rt>, <Rt2>, <CRm>"];
}
pub(crate) struct MRS;
impl MRS {
    const ENC: &'static [&'static str] = &[
        "MRS{<c>}{<q>} <Rd>, <spec_reg>",
        "MRS{<c>}{<q>} <Rd>, <banked_reg>",
    ];
}
pub(crate) struct MSR;
impl MSR {
    const ENC: &'static [&'static str] = &[
        "MSR{<c>}{<q>} <banked_reg>, <Rn>",
        "MSR{<c>}{<q>} <spec_reg>, #<imm>",
        "MSR{<c>}{<q>} <spec_reg>, <Rn>",
    ];
}
pub(crate) struct MUL;
impl MUL {
    const ENC: &'static [&'static str] = &["MUL{<c>}{<q>} <Rd>, <Rn>{, <Rm>}"];
}
pub(crate) struct MULS;
impl MULS {
    const ENC: &'static [&'static str] = &["MULS{<c>}{<q>} <Rd>, <Rn>{, <Rm>}"];
}
pub(crate) struct MVN;
impl MVN {
    const ENC: &'static [&'static str] = &[
        "MVN{<c>}{<q>} <Rd>, #<const>",
        "MVN{<c>}{<q>} <Rd>, <Rm>, RRX",
        "MVN{<c>}{<q>} <Rd>, <Rm> {, <shift> #<amount>}",
        "MVN{<c>}{<q>} <Rd>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct MVNS;
impl MVNS {
    const ENC: &'static [&'static str] = &[
        "MVNS{<c>}{<q>} <Rd>, #<const>",
        "MVNS{<c>}{<q>} <Rd>, <Rm>, RRX",
        "MVNS{<c>}{<q>} <Rd>, <Rm> {, <shift> #<amount>}",
        "MVNS{<c>}{<q>} <Rd>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct NOP;
impl NOP {
    const ENC: &'static [&'static str] = &["NOP{<c>}{<q>}"];
}
pub(crate) struct ORR;
impl ORR {
    const ENC: &'static [&'static str] = &[
        "ORR{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "ORR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "ORR{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "ORR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct ORRS;
impl ORRS {
    const ENC: &'static [&'static str] = &[
        "ORRS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "ORRS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "ORRS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "ORRS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct PKHBT;
impl PKHBT {
    const ENC: &'static [&'static str] = &["PKHBT{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, LSL #<imm>}"];
}
pub(crate) struct PKHTB;
impl PKHTB {
    const ENC: &'static [&'static str] = &["PKHTB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ASR #<imm>}"];
}
pub(crate) struct PLD;
impl PLD {
    const ENC: &'static [&'static str] = &[
        "PLD{<c>}{<q>} [<Rn> {, #{+/-}<imm>}]",
        "PLD{<c>}{<q>} <label>",
        "PLD{<c>}{<q>} [<Rn>, {+/-}<Rm> {, <shift> #<amount>}]",
        "PLD{<c>}{<q>} [<Rn>, {+/-}<Rm> , RRX]",
    ];
}
pub(crate) struct PLDW;
impl PLDW {
    const ENC: &'static [&'static str] = &[
        "PLDW{<c>}{<q>} [<Rn> {, #{+/-}<imm>}]",
        "PLDW{<c>}{<q>} [<Rn>, {+/-}<Rm> {, <shift> #<amount>}]",
        "PLDW{<c>}{<q>} [<Rn>, {+/-}<Rm> , RRX]",
    ];
}
pub(crate) struct PLI;
impl PLI {
    const ENC: &'static [&'static str] = &[
        "PLI{<c>}{<q>} [<Rn> {, #{+/-}<imm>}]",
        "PLI{<c>}{<q>} [<Rn>, {+/-}<Rm> , RRX]",
        "PLI{<c>}{<q>} [<Rn>, {+/-}<Rm> {, <shift> #<amount>}]",
    ];
}
pub(crate) struct PSSBB;
impl PSSBB {
    const ENC: &'static [&'static str] = &["PSSBB{<q>}"];
}
pub(crate) struct QADD;
impl QADD {
    const ENC: &'static [&'static str] = &["QADD{<c>}{<q>} {<Rd>,} <Rm>, <Rn>"];
}
pub(crate) struct QADD16;
impl QADD16 {
    const ENC: &'static [&'static str] = &["QADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct QADD8;
impl QADD8 {
    const ENC: &'static [&'static str] = &["QADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct QASX;
impl QASX {
    const ENC: &'static [&'static str] = &["QASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct QDADD;
impl QDADD {
    const ENC: &'static [&'static str] = &["QDADD{<c>}{<q>} {<Rd>,} <Rm>, <Rn>"];
}
pub(crate) struct QDSUB;
impl QDSUB {
    const ENC: &'static [&'static str] = &["QDSUB{<c>}{<q>} {<Rd>,} <Rm>, <Rn>"];
}
pub(crate) struct QSAX;
impl QSAX {
    const ENC: &'static [&'static str] = &["QSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct QSUB;
impl QSUB {
    const ENC: &'static [&'static str] = &["QSUB{<c>}{<q>} {<Rd>,} <Rm>, <Rn>"];
}
pub(crate) struct QSUB16;
impl QSUB16 {
    const ENC: &'static [&'static str] = &["QSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct QSUB8;
impl QSUB8 {
    const ENC: &'static [&'static str] = &["QSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct RBIT;
impl RBIT {
    const ENC: &'static [&'static str] = &["RBIT{<c>}{<q>} <Rd>, <Rm>"];
}
pub(crate) struct REV;
impl REV {
    const ENC: &'static [&'static str] = &["REV{<c>}{<q>} <Rd>, <Rm>"];
}
pub(crate) struct REV16;
impl REV16 {
    const ENC: &'static [&'static str] = &["REV16{<c>}{<q>} <Rd>, <Rm>"];
}
pub(crate) struct REVSH;
impl REVSH {
    const ENC: &'static [&'static str] = &["REVSH{<c>}{<q>} <Rd>, <Rm>"];
}
pub(crate) struct RFE;
impl RFE {
    const ENC: &'static [&'static str] = &[
        "RFEDA{<c>}{<q>} <Rn>{!}",
        "RFEDB{<c>}{<q>} <Rn>{!}",
        "RFE{IA}{<c>}{<q>} <Rn>{!}",
        "RFEIB{<c>}{<q>} <Rn>{!}",
    ];
}
pub(crate) struct RSB;
impl RSB {
    const ENC: &'static [&'static str] = &[
        "RSB{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "RSB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "RSB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "RSB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct RSBS;
impl RSBS {
    const ENC: &'static [&'static str] = &[
        "RSBS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "RSBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "RSBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "RSBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct RSC;
impl RSC {
    const ENC: &'static [&'static str] = &[
        "RSC{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "RSC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "RSC{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "RSC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct RSCS;
impl RSCS {
    const ENC: &'static [&'static str] = &[
        "RSCS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "RSCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "RSCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "RSCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct SADD16;
impl SADD16 {
    const ENC: &'static [&'static str] = &["SADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SADD8;
impl SADD8 {
    const ENC: &'static [&'static str] = &["SADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SASX;
impl SASX {
    const ENC: &'static [&'static str] = &["SASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SB;
impl SB {
    const ENC: &'static [&'static str] = &["SB{<q>}"];
}
pub(crate) struct SBC;
impl SBC {
    const ENC: &'static [&'static str] = &[
        "SBC{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "SBC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "SBC{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "SBC{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct SBCS;
impl SBCS {
    const ENC: &'static [&'static str] = &[
        "SBCS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "SBCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "SBCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "SBCS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
    ];
}
pub(crate) struct SBFX;
impl SBFX {
    const ENC: &'static [&'static str] = &["SBFX{<c>}{<q>} <Rd>, <Rn>, #<lsb>, #<width>"];
}
pub(crate) struct SDIV;
impl SDIV {
    const ENC: &'static [&'static str] = &["SDIV{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SEL;
impl SEL {
    const ENC: &'static [&'static str] = &["SEL{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SETEND;
impl SETEND {
    const ENC: &'static [&'static str] = &["SETEND{<q>} <endian_specifier>"];
}
pub(crate) struct SETPAN;
impl SETPAN {
    const ENC: &'static [&'static str] = &["SETPAN{<q>} #<imm>"];
}
pub(crate) struct SEV;
impl SEV {
    const ENC: &'static [&'static str] = &["SEV{<c>}{<q>}"];
}
pub(crate) struct SEVL;
impl SEVL {
    const ENC: &'static [&'static str] = &["SEVL{<c>}{<q>}"];
}
pub(crate) struct SHADD16;
impl SHADD16 {
    const ENC: &'static [&'static str] = &["SHADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SHADD8;
impl SHADD8 {
    const ENC: &'static [&'static str] = &["SHADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SHASX;
impl SHASX {
    const ENC: &'static [&'static str] = &["SHASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SHSAX;
impl SHSAX {
    const ENC: &'static [&'static str] = &["SHSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SHSUB16;
impl SHSUB16 {
    const ENC: &'static [&'static str] = &["SHSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SHSUB8;
impl SHSUB8 {
    const ENC: &'static [&'static str] = &["SHSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMC;
impl SMC {
    const ENC: &'static [&'static str] = &["SMC{<c>}{<q>} {#}<imm4>"];
}
pub(crate) struct SMLABB;
impl SMLABB {
    const ENC: &'static [&'static str] = &["SMLABB{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLABT;
impl SMLABT {
    const ENC: &'static [&'static str] = &["SMLABT{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLAD;
impl SMLAD {
    const ENC: &'static [&'static str] = &["SMLAD{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLADX;
impl SMLADX {
    const ENC: &'static [&'static str] = &["SMLADX{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLAL;
impl SMLAL {
    const ENC: &'static [&'static str] = &["SMLAL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLALBB;
impl SMLALBB {
    const ENC: &'static [&'static str] = &["SMLALBB{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLALBT;
impl SMLALBT {
    const ENC: &'static [&'static str] = &["SMLALBT{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLALD;
impl SMLALD {
    const ENC: &'static [&'static str] = &["SMLALD{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLALDX;
impl SMLALDX {
    const ENC: &'static [&'static str] = &["SMLALDX{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLALS;
impl SMLALS {
    const ENC: &'static [&'static str] = &["SMLALS{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLALTB;
impl SMLALTB {
    const ENC: &'static [&'static str] = &["SMLALTB{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLALTT;
impl SMLALTT {
    const ENC: &'static [&'static str] = &["SMLALTT{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLATB;
impl SMLATB {
    const ENC: &'static [&'static str] = &["SMLATB{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLATT;
impl SMLATT {
    const ENC: &'static [&'static str] = &["SMLATT{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLAWB;
impl SMLAWB {
    const ENC: &'static [&'static str] = &["SMLAWB{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLAWT;
impl SMLAWT {
    const ENC: &'static [&'static str] = &["SMLAWT{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLSD;
impl SMLSD {
    const ENC: &'static [&'static str] = &["SMLSD{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLSDX;
impl SMLSDX {
    const ENC: &'static [&'static str] = &["SMLSDX{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMLSLD;
impl SMLSLD {
    const ENC: &'static [&'static str] = &["SMLSLD{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMLSLDX;
impl SMLSLDX {
    const ENC: &'static [&'static str] = &["SMLSLDX{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMMLA;
impl SMMLA {
    const ENC: &'static [&'static str] = &["SMMLA{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMMLAR;
impl SMMLAR {
    const ENC: &'static [&'static str] = &["SMMLAR{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMMLS;
impl SMMLS {
    const ENC: &'static [&'static str] = &["SMMLS{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMMLSR;
impl SMMLSR {
    const ENC: &'static [&'static str] = &["SMMLSR{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct SMMUL;
impl SMMUL {
    const ENC: &'static [&'static str] = &["SMMUL{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMMULR;
impl SMMULR {
    const ENC: &'static [&'static str] = &["SMMULR{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMUAD;
impl SMUAD {
    const ENC: &'static [&'static str] = &["SMUAD{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMUADX;
impl SMUADX {
    const ENC: &'static [&'static str] = &["SMUADX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMULBB;
impl SMULBB {
    const ENC: &'static [&'static str] = &["SMULBB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMULBT;
impl SMULBT {
    const ENC: &'static [&'static str] = &["SMULBT{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMULL;
impl SMULL {
    const ENC: &'static [&'static str] = &["SMULL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMULLS;
impl SMULLS {
    const ENC: &'static [&'static str] = &["SMULLS{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct SMULTB;
impl SMULTB {
    const ENC: &'static [&'static str] = &["SMULTB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMULTT;
impl SMULTT {
    const ENC: &'static [&'static str] = &["SMULTT{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMULWB;
impl SMULWB {
    const ENC: &'static [&'static str] = &["SMULWB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMULWT;
impl SMULWT {
    const ENC: &'static [&'static str] = &["SMULWT{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMUSD;
impl SMUSD {
    const ENC: &'static [&'static str] = &["SMUSD{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SMUSDX;
impl SMUSDX {
    const ENC: &'static [&'static str] = &["SMUSDX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SRS;
impl SRS {
    const ENC: &'static [&'static str] = &["SRS{IA}{<c>}{<q>} SP{!}, #<mode>"];
}
pub(crate) struct SRSDA;
impl SRSDA {
    const ENC: &'static [&'static str] = &["SRSDA{<c>}{<q>} SP{!}, #<mode>"];
}
pub(crate) struct SRSDB;
impl SRSDB {
    const ENC: &'static [&'static str] = &["SRSDB{<c>}{<q>} SP{!}, #<mode>"];
}
pub(crate) struct SRSIB;
impl SRSIB {
    const ENC: &'static [&'static str] = &["SRSIB{<c>}{<q>} SP{!}, #<mode>"];
}
pub(crate) struct SSAT;
impl SSAT {
    const ENC: &'static [&'static str] = &[
        "SSAT{<c>}{<q>} <Rd>, #<imm>, <Rn>, ASR #<amount>",
        "SSAT{<c>}{<q>} <Rd>, #<imm>, <Rn> {, LSL #<amount>}",
    ];
}
pub(crate) struct SSAT16;
impl SSAT16 {
    const ENC: &'static [&'static str] = &["SSAT16{<c>}{<q>} <Rd>, #<imm>, <Rn>"];
}
pub(crate) struct SSAX;
impl SSAX {
    const ENC: &'static [&'static str] = &["SSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SSBB;
impl SSBB {
    const ENC: &'static [&'static str] = &["SSBB{<q>}"];
}
pub(crate) struct SSUB16;
impl SSUB16 {
    const ENC: &'static [&'static str] = &["SSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct SSUB8;
impl SSUB8 {
    const ENC: &'static [&'static str] = &["SSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct STC;
impl STC {
    const ENC: &'static [&'static str] = &[
        "STC{<c>}{<q>} p14, c5, [<Rn>{, #{+/-}<imm>}]",
        "STC{<c>}{<q>} p14, c5, [<Rn>], #{+/-}<imm>",
        "STC{<c>}{<q>} p14, c5, [<Rn>, #{+/-}<imm>]!",
        "STC{<c>}{<q>} p14, c5, [<Rn>], <option>",
    ];
}
pub(crate) struct STL;
impl STL {
    const ENC: &'static [&'static str] = &["STL{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct STLB;
impl STLB {
    const ENC: &'static [&'static str] = &["STLB{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct STLEX;
impl STLEX {
    const ENC: &'static [&'static str] = &["STLEX{<c>}{<q>} <Rd>, <Rt>, [<Rn>]"];
}
pub(crate) struct STLEXB;
impl STLEXB {
    const ENC: &'static [&'static str] = &["STLEXB{<c>}{<q>} <Rd>, <Rt>, [<Rn>]"];
}
pub(crate) struct STLEXD;
impl STLEXD {
    const ENC: &'static [&'static str] = &["STLEXD{<c>}{<q>} <Rd>, <Rt>, <Rt2>, [<Rn>]"];
}
pub(crate) struct STLEXH;
impl STLEXH {
    const ENC: &'static [&'static str] = &["STLEXH{<c>}{<q>} <Rd>, <Rt>, [<Rn>]"];
}
pub(crate) struct STLH;
impl STLH {
    const ENC: &'static [&'static str] = &["STLH{<c>}{<q>} <Rt>, [<Rn>]"];
}
pub(crate) struct STM;
impl STM {
    const ENC: &'static [&'static str] = &[
        "PUSH{<c>}{<q>} <registers>",
        "STM{IA}{<c>}{<q>} <Rn>{!}, <registers>",
        "STMDA{<c>}{<q>} <Rn>{!}, <registers>",
        "STMDB{<c>}{<q>} <Rn>{!}, <registers>",
        "STMIB{<c>}{<q>} <Rn>{!}, <registers>",
        "STM{<amode>}{<c>}{<q>} <Rn>, <registers>^",
    ];
}
pub(crate) struct STR;
impl STR {
    const ENC: &'static [&'static str] = &[
        "PUSH{<c>}{<q>} <single_register_list>",
        "STR{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]",
        "STR{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>",
        "STR{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!",
        "STR{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]",
        "STR{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}",
        "STR{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]!",
    ];
}
pub(crate) struct STRB;
impl STRB {
    const ENC: &'static [&'static str] = &[
        "STRB{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]",
        "STRB{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>",
        "STRB{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!",
        "STRB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]",
        "STRB{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>{, <shift>}",
        "STRB{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>{, <shift>}]!",
    ];
}
pub(crate) struct STRBT;
impl STRBT {
    const ENC: &'static [&'static str] = &["STRBT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}"];
}
pub(crate) struct STRD;
impl STRD {
    const ENC: &'static [&'static str] = &[
        "STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn> {, #{+/-}<imm>}]",
        "STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>], #{+/-}<imm>",
        "STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, #{+/-}<imm>]!",
        "STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, {+/-}<Rm>]",
        "STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>], {+/-}<Rm>",
        "STRD{<c>}{<q>} <Rt>, <Rt2>, [<Rn>, {+/-}<Rm>]!",
    ];
}
pub(crate) struct STREX;
impl STREX {
    const ENC: &'static [&'static str] = &["STREX{<c>}{<q>} <Rd>, <Rt>, [<Rn> {, {#}<imm>}]"];
}
pub(crate) struct STREXB;
impl STREXB {
    const ENC: &'static [&'static str] = &["STREXB{<c>}{<q>} <Rd>, <Rt>, [<Rn>]"];
}
pub(crate) struct STREXD;
impl STREXD {
    const ENC: &'static [&'static str] = &["STREXD{<c>}{<q>} <Rd>, <Rt>, <Rt2>, [<Rn>]"];
}
pub(crate) struct STREXH;
impl STREXH {
    const ENC: &'static [&'static str] = &["STREXH{<c>}{<q>} <Rd>, <Rt>, [<Rn>]"];
}
pub(crate) struct STRH;
impl STRH {
    const ENC: &'static [&'static str] = &[
        "STRH{<c>}{<q>} <Rt>, [<Rn> {, #{+/-}<imm>}]",
        "STRH{<c>}{<q>} <Rt>, [<Rn>], #{+/-}<imm>",
        "STRH{<c>}{<q>} <Rt>, [<Rn>, #{+/-}<imm>]!",
        "STRH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]",
        "STRH{<c>}{<q>} <Rt>, [<Rn>], {+/-}<Rm>",
        "STRH{<c>}{<q>} <Rt>, [<Rn>, {+/-}<Rm>]!",
    ];
}
pub(crate) struct STRHT;
impl STRHT {
    const ENC: &'static [&'static str] = &["STRHT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}"];
}
pub(crate) struct STRT;
impl STRT {
    const ENC: &'static [&'static str] = &["STRT{<c>}{<q>} <Rt>, [<Rn>] {, #{+/-}<imm>}"];
}
pub(crate) struct SUB;
impl SUB {
    const ENC: &'static [&'static str] = &[
        "SUB{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "SUB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "SUB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "SUB{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
        "SUB{<c>}{<q>} {<Rd>,} SP, #<const>",
        "SUB{<c>}{<q>} {<Rd>,} SP, <Rm> , RRX",
        "SUB{<c>}{<q>} {<Rd>,} SP, <Rm> {, <shift> #<amount>}",
    ];
}
pub(crate) struct SUBS;
impl SUBS {
    const ENC: &'static [&'static str] = &[
        "SUBS{<c>}{<q>} {<Rd>,} <Rn>, #<const>",
        "SUBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, RRX",
        "SUBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, <shift> #<amount>}",
        "SUBS{<c>}{<q>} {<Rd>,} <Rn>, <Rm>, <shift> <Rs>",
        "SUBS{<c>}{<q>} {<Rd>,} SP, #<const>",
        "SUBS{<c>}{<q>} {<Rd>,} SP, <Rm> , RRX",
        "SUBS{<c>}{<q>} {<Rd>,} SP, <Rm> {, <shift> #<amount>}",
    ];
}
pub(crate) struct SVC;
impl SVC {
    const ENC: &'static [&'static str] = &["SVC{<c>}{<q>} {#}<imm>"];
}
pub(crate) struct SXTAB;
impl SXTAB {
    const ENC: &'static [&'static str] = &["SXTAB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}"];
}
pub(crate) struct SXTAB16;
impl SXTAB16 {
    const ENC: &'static [&'static str] =
        &["SXTAB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}"];
}
pub(crate) struct SXTAH;
impl SXTAH {
    const ENC: &'static [&'static str] = &["SXTAH{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}"];
}
pub(crate) struct SXTB;
impl SXTB {
    const ENC: &'static [&'static str] = &["SXTB{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}"];
}
pub(crate) struct SXTB16;
impl SXTB16 {
    const ENC: &'static [&'static str] = &["SXTB16{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}"];
}
pub(crate) struct SXTH;
impl SXTH {
    const ENC: &'static [&'static str] = &["SXTH{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}"];
}
pub(crate) struct TEQ;
impl TEQ {
    const ENC: &'static [&'static str] = &[
        "TEQ{<c>}{<q>} <Rn>, #<const>",
        "TEQ{<c>}{<q>} <Rn>, <Rm>, RRX",
        "TEQ{<c>}{<q>} <Rn>, <Rm> {, <shift> #<amount>}",
        "TEQ{<c>}{<q>} <Rn>, <Rm>, <type> <Rs>",
    ];
}
pub(crate) struct TSB;
impl TSB {
    const ENC: &'static [&'static str] = &["TSB{<c>}{<q>} CSYNC"];
}
pub(crate) struct TST;
impl TST {
    const ENC: &'static [&'static str] = &[
        "TST{<c>}{<q>} <Rn>, #<const>",
        "TST{<c>}{<q>} <Rn>, <Rm>, RRX",
        "TST{<c>}{<q>} <Rn>, <Rm> {, <shift> #<amount>}",
        "TST{<c>}{<q>} <Rn>, <Rm>, <type> <Rs>",
    ];
}
pub(crate) struct UADD16;
impl UADD16 {
    const ENC: &'static [&'static str] = &["UADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UADD8;
impl UADD8 {
    const ENC: &'static [&'static str] = &["UADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UASX;
impl UASX {
    const ENC: &'static [&'static str] = &["UASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UBFX;
impl UBFX {
    const ENC: &'static [&'static str] = &["UBFX{<c>}{<q>} <Rd>, <Rn>, #<lsb>, #<width>"];
}
pub(crate) struct UDF;
impl UDF {
    const ENC: &'static [&'static str] = &["UDF{<c>}{<q>} {#}<imm>"];
}
pub(crate) struct UDIV;
impl UDIV {
    const ENC: &'static [&'static str] = &["UDIV{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UHADD16;
impl UHADD16 {
    const ENC: &'static [&'static str] = &["UHADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UHADD8;
impl UHADD8 {
    const ENC: &'static [&'static str] = &["UHADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UHASX;
impl UHASX {
    const ENC: &'static [&'static str] = &["UHASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UHSAX;
impl UHSAX {
    const ENC: &'static [&'static str] = &["UHSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UHSUB16;
impl UHSUB16 {
    const ENC: &'static [&'static str] = &["UHSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UHSUB8;
impl UHSUB8 {
    const ENC: &'static [&'static str] = &["UHSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UMAAL;
impl UMAAL {
    const ENC: &'static [&'static str] = &["UMAAL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct UMLAL;
impl UMLAL {
    const ENC: &'static [&'static str] = &["UMLAL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct UMLALS;
impl UMLALS {
    const ENC: &'static [&'static str] = &["UMLALS{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct UMULL;
impl UMULL {
    const ENC: &'static [&'static str] = &["UMULL{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct UMULLS;
impl UMULLS {
    const ENC: &'static [&'static str] = &["UMULLS{<c>}{<q>} <RdLo>, <RdHi>, <Rn>, <Rm>"];
}
pub(crate) struct UQADD16;
impl UQADD16 {
    const ENC: &'static [&'static str] = &["UQADD16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UQADD8;
impl UQADD8 {
    const ENC: &'static [&'static str] = &["UQADD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UQASX;
impl UQASX {
    const ENC: &'static [&'static str] = &["UQASX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UQSAX;
impl UQSAX {
    const ENC: &'static [&'static str] = &["UQSAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UQSUB16;
impl UQSUB16 {
    const ENC: &'static [&'static str] = &["UQSUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UQSUB8;
impl UQSUB8 {
    const ENC: &'static [&'static str] = &["UQSUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct USAD8;
impl USAD8 {
    const ENC: &'static [&'static str] = &["USAD8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct USADA8;
impl USADA8 {
    const ENC: &'static [&'static str] = &["USADA8{<c>}{<q>} <Rd>, <Rn>, <Rm>, <Ra>"];
}
pub(crate) struct USAT;
impl USAT {
    const ENC: &'static [&'static str] = &[
        "USAT{<c>}{<q>} <Rd>, #<imm>, <Rn>, ASR #<amount>",
        "USAT{<c>}{<q>} <Rd>, #<imm>, <Rn> {, LSL #<amount>}",
    ];
}
pub(crate) struct USAT16;
impl USAT16 {
    const ENC: &'static [&'static str] = &["USAT16{<c>}{<q>} <Rd>, #<imm>, <Rn>"];
}
pub(crate) struct USAX;
impl USAX {
    const ENC: &'static [&'static str] = &["USAX{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct USUB16;
impl USUB16 {
    const ENC: &'static [&'static str] = &["USUB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct USUB8;
impl USUB8 {
    const ENC: &'static [&'static str] = &["USUB8{<c>}{<q>} {<Rd>,} <Rn>, <Rm>"];
}
pub(crate) struct UXTAB;
impl UXTAB {
    const ENC: &'static [&'static str] = &["UXTAB{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}"];
}
pub(crate) struct UXTAB16;
impl UXTAB16 {
    const ENC: &'static [&'static str] =
        &["UXTAB16{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}"];
}
pub(crate) struct UXTAH;
impl UXTAH {
    const ENC: &'static [&'static str] = &["UXTAH{<c>}{<q>} {<Rd>,} <Rn>, <Rm> {, ROR #<amount>}"];
}
pub(crate) struct UXTB;
impl UXTB {
    const ENC: &'static [&'static str] = &["UXTB{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}"];
}
pub(crate) struct UXTB16;
impl UXTB16 {
    const ENC: &'static [&'static str] = &["UXTB16{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}"];
}
pub(crate) struct UXTH;
impl UXTH {
    const ENC: &'static [&'static str] = &["UXTH{<c>}{<q>} {<Rd>,} <Rm> {, ROR #<amount>}"];
}
pub(crate) struct WFE;
impl WFE {
    const ENC: &'static [&'static str] = &["WFE{<c>}{<q>}"];
}
pub(crate) struct WFI;
impl WFI {
    const ENC: &'static [&'static str] = &["WFI{<c>}{<q>}"];
}
pub(crate) struct YIELD;
impl YIELD {
    const ENC: &'static [&'static str] = &["YIELD{<c>}{<q>}"];
}
