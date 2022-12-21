use super::encoding;
use super::types::*;

use super::macros::functions;


functions! {
    cas_32(rs: Register32, rt: Register32, rn: Register64OrSp) => CAS_C32_comswap;
    cas_64(rs: Register64, rt: Register64, rn: Register64OrSp) => CAS_C64_comswap;
    casa_32(rs: Register32, rt: Register32, rn: Register64OrSp) => CASA_C32_comswap;
    casa_64(rs: Register64, rt: Register64, rn: Register64OrSp) => CASA_C64_comswap;
    casab(rs: Register32, rt: Register32, rn: Register64OrSp) => CASAB_C32_comswap;
    casah(rs: Register32, rt: Register32, rn: Register64OrSp) => CASAH_C32_comswap;
    casal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => CASAL_C32_comswap;
    casal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => CASAL_C64_comswap;
    casalb(rs: Register32, rt: Register32, rn: Register64OrSp) => CASALB_C32_comswap;
    casalh(rs: Register32, rt: Register32, rn: Register64OrSp) => CASALH_C32_comswap;
    casb(rs: Register32, rt: Register32, rn: Register64OrSp) => CASB_C32_comswap;
    cash(rs: Register32, rt: Register32, rn: Register64OrSp) => CASH_C32_comswap;
    casl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => CASL_C32_comswap;
    casl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => CASL_C64_comswap;
    caslb(rs: Register32, rt: Register32, rn: Register64OrSp) => CASLB_C32_comswap;
    caslh(rs: Register32, rt: Register32, rn: Register64OrSp) => CASLH_C32_comswap;
    casp_32(rs: Register32, rt: Register32, rn: Register64OrSp) => CASP_CP32_comswappr;
    casp_64(rs: Register64, rt: Register64, rn: Register64OrSp) => CASP_CP64_comswappr;
    caspa_32(rs: Register32, rt: Register32, rn: Register64OrSp) => CASPA_CP32_comswappr;
    caspa_64(rs: Register64, rt: Register64, rn: Register64OrSp) => CASPA_CP64_comswappr;
    caspal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => CASPAL_CP32_comswappr;
    caspal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => CASPAL_CP64_comswappr;
    caspl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => CASPL_CP32_comswappr;
    caspl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => CASPL_CP64_comswappr;
    ldadd_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADD_32_memop;
    ldadd_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDADD_64_memop;
    ldadda_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDA_32_memop;
    ldadda_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDADDA_64_memop;
    ldaddab(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDAB_32_memop;
    ldaddah(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDAH_32_memop;
    ldaddal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDAL_32_memop;
    ldaddal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDADDAL_64_memop;
    ldaddalb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDALB_32_memop;
    ldaddalh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDALH_32_memop;
    ldaddb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDB_32_memop;
    ldaddh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDH_32_memop;
    ldaddl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDL_32_memop;
    ldaddl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDADDL_64_memop;
    ldaddlb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDLB_32_memop;
    ldaddlh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDADDLH_32_memop;
    ldclr_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLR_32_memop;
    ldclr_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDCLR_64_memop;
    ldclra_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRA_32_memop;
    ldclra_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDCLRA_64_memop;
    ldclrab(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRAB_32_memop;
    ldclrah(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRAH_32_memop;
    ldclral_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRAL_32_memop;
    ldclral_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDCLRAL_64_memop;
    ldclralb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRALB_32_memop;
    ldclralh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRALH_32_memop;
    ldclrb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRB_32_memop;
    ldclrh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRH_32_memop;
    ldclrl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRL_32_memop;
    ldclrl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDCLRL_64_memop;
    ldclrlb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRLB_32_memop;
    ldclrlh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDCLRLH_32_memop;
    ldeor_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEOR_32_memop;
    ldeor_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDEOR_64_memop;
    ldeora_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORA_32_memop;
    ldeora_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDEORA_64_memop;
    ldeorab(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORAB_32_memop;
    ldeorah(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORAH_32_memop;
    ldeoral_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORAL_32_memop;
    ldeoral_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDEORAL_64_memop;
    ldeoralb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORALB_32_memop;
    ldeoralh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORALH_32_memop;
    ldeorb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORB_32_memop;
    ldeorh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORH_32_memop;
    ldeorl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORL_32_memop;
    ldeorl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDEORL_64_memop;
    ldeorlb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORLB_32_memop;
    ldeorlh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDEORLH_32_memop;
    ldset_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSET_32_memop;
    ldset_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSET_64_memop;
    ldseta_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETA_32_memop;
    ldseta_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSETA_64_memop;
    ldsetab(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETAB_32_memop;
    ldsetah(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETAH_32_memop;
    ldsetal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETAL_32_memop;
    ldsetal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSETAL_64_memop;
    ldsetalb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETALB_32_memop;
    ldsetalh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETALH_32_memop;
    ldsetb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETB_32_memop;
    ldseth(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETH_32_memop;
    ldsetl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETL_32_memop;
    ldsetl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSETL_64_memop;
    ldsetlb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETLB_32_memop;
    ldsetlh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSETLH_32_memop;
    ldsmax_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAX_32_memop;
    ldsmax_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSMAX_64_memop;
    ldsmaxa_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXA_32_memop;
    ldsmaxa_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSMAXA_64_memop;
    ldsmaxab(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXAB_32_memop;
    ldsmaxah(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXAH_32_memop;
    ldsmaxal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXAL_32_memop;
    ldsmaxal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSMAXAL_64_memop;
    ldsmaxalb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXALB_32_memop;
    ldsmaxalh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXALH_32_memop;
    ldsmaxb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXB_32_memop;
    ldsmaxh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXH_32_memop;
    ldsmaxl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXL_32_memop;
    ldsmaxl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSMAXL_64_memop;
    ldsmaxlb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXLB_32_memop;
    ldsmaxlh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMAXLH_32_memop;
    ldsmin_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMIN_32_memop;
    ldsmin_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSMIN_64_memop;
    ldsmina_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINA_32_memop;
    ldsmina_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSMINA_64_memop;
    ldsminab(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINAB_32_memop;
    ldsminah(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINAH_32_memop;
    ldsminal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINAL_32_memop;
    ldsminal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSMINAL_64_memop;
    ldsminalb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINALB_32_memop;
    ldsminalh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINALH_32_memop;
    ldsminb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINB_32_memop;
    ldsminh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINH_32_memop;
    ldsminl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINL_32_memop;
    ldsminl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDSMINL_64_memop;
    ldsminlb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINLB_32_memop;
    ldsminlh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDSMINLH_32_memop;
    ldumax_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAX_32_memop;
    ldumax_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDUMAX_64_memop;
    ldumaxa_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXA_32_memop;
    ldumaxa_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDUMAXA_64_memop;
    ldumaxab(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXAB_32_memop;
    ldumaxah(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXAH_32_memop;
    ldumaxal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXAL_32_memop;
    ldumaxal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDUMAXAL_64_memop;
    ldumaxalb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXALB_32_memop;
    ldumaxalh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXALH_32_memop;
    ldumaxb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXB_32_memop;
    ldumaxh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXH_32_memop;
    ldumaxl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXL_32_memop;
    ldumaxl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDUMAXL_64_memop;
    ldumaxlb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXLB_32_memop;
    ldumaxlh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMAXLH_32_memop;
    ldumin_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMIN_32_memop;
    ldumin_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDUMIN_64_memop;
    ldumina_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINA_32_memop;
    ldumina_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDUMINA_64_memop;
    lduminab(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINAB_32_memop;
    lduminah(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINAH_32_memop;
    lduminal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINAL_32_memop;
    lduminal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDUMINAL_64_memop;
    lduminalb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINALB_32_memop;
    lduminalh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINALH_32_memop;
    lduminb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINB_32_memop;
    lduminh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINH_32_memop;
    lduminl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINL_32_memop;
    lduminl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => LDUMINL_64_memop;
    lduminlb(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINLB_32_memop;
    lduminlh(rs: Register32, rt: Register32, rn: Register64OrSp) => LDUMINLH_32_memop;
    stadd_32(rs: Register32, rn: Register64OrSp) => STADD_LDADD_32_memop;
    stadd_64(rs: Register64, rn: Register64OrSp) => STADD_LDADD_64_memop;
    staddb(rs: Register32, rn: Register64OrSp) => STADDB_LDADDB_32_memop;
    staddh(rs: Register32, rn: Register64OrSp) => STADDH_LDADDH_32_memop;
    staddl_32(rs: Register32, rn: Register64OrSp) => STADDL_LDADDL_32_memop;
    staddl_64(rs: Register64, rn: Register64OrSp) => STADDL_LDADDL_64_memop;
    staddlb(rs: Register32, rn: Register64OrSp) => STADDLB_LDADDLB_32_memop;
    staddlh(rs: Register32, rn: Register64OrSp) => STADDLH_LDADDLH_32_memop;
    stclr_32(rs: Register32, rn: Register64OrSp) => STCLR_LDCLR_32_memop;
    stclr_64(rs: Register64, rn: Register64OrSp) => STCLR_LDCLR_64_memop;
    stclrb(rs: Register32, rn: Register64OrSp) => STCLRB_LDCLRB_32_memop;
    stclrh(rs: Register32, rn: Register64OrSp) => STCLRH_LDCLRH_32_memop;
    stclrl_32(rs: Register32, rn: Register64OrSp) => STCLRL_LDCLRL_32_memop;
    stclrl_64(rs: Register64, rn: Register64OrSp) => STCLRL_LDCLRL_64_memop;
    stclrlb(rs: Register32, rn: Register64OrSp) => STCLRLB_LDCLRLB_32_memop;
    stclrlh(rs: Register32, rn: Register64OrSp) => STCLRLH_LDCLRLH_32_memop;
    steor_32(rs: Register32, rn: Register64OrSp) => STEOR_LDEOR_32_memop;
    steor_64(rs: Register64, rn: Register64OrSp) => STEOR_LDEOR_64_memop;
    steorb(rs: Register32, rn: Register64OrSp) => STEORB_LDEORB_32_memop;
    steorh(rs: Register32, rn: Register64OrSp) => STEORH_LDEORH_32_memop;
    steorl_32(rs: Register32, rn: Register64OrSp) => STEORL_LDEORL_32_memop;
    steorl_64(rs: Register64, rn: Register64OrSp) => STEORL_LDEORL_64_memop;
    steorlb(rs: Register32, rn: Register64OrSp) => STEORLB_LDEORLB_32_memop;
    steorlh(rs: Register32, rn: Register64OrSp) => STEORLH_LDEORLH_32_memop;
    stset_32(rs: Register32, rn: Register64OrSp) => STSET_LDSET_32_memop;
    stset_64(rs: Register64, rn: Register64OrSp) => STSET_LDSET_64_memop;
    stsetb(rs: Register32, rn: Register64OrSp) => STSETB_LDSETB_32_memop;
    stseth(rs: Register32, rn: Register64OrSp) => STSETH_LDSETH_32_memop;
    stsetl_32(rs: Register32, rn: Register64OrSp) => STSETL_LDSETL_32_memop;
    stsetl_64(rs: Register64, rn: Register64OrSp) => STSETL_LDSETL_64_memop;
    stsetlb(rs: Register32, rn: Register64OrSp) => STSETLB_LDSETLB_32_memop;
    stsetlh(rs: Register32, rn: Register64OrSp) => STSETLH_LDSETLH_32_memop;
    stsmax_32(rs: Register32, rn: Register64OrSp) => STSMAX_LDSMAX_32_memop;
    stsmax_64(rs: Register64, rn: Register64OrSp) => STSMAX_LDSMAX_64_memop;
    stsmaxb(rs: Register32, rn: Register64OrSp) => STSMAXB_LDSMAXB_32_memop;
    stsmaxh(rs: Register32, rn: Register64OrSp) => STSMAXH_LDSMAXH_32_memop;
    stsmaxl_32(rs: Register32, rn: Register64OrSp) => STSMAXL_LDSMAXL_32_memop;
    stsmaxl_64(rs: Register64, rn: Register64OrSp) => STSMAXL_LDSMAXL_64_memop;
    stsmaxlb(rs: Register32, rn: Register64OrSp) => STSMAXLB_LDSMAXLB_32_memop;
    stsmaxlh(rs: Register32, rn: Register64OrSp) => STSMAXLH_LDSMAXLH_32_memop;
    stsmin_32(rs: Register32, rn: Register64OrSp) => STSMIN_LDSMIN_32_memop;
    stsmin_64(rs: Register64, rn: Register64OrSp) => STSMIN_LDSMIN_64_memop;
    stsminb(rs: Register32, rn: Register64OrSp) => STSMINB_LDSMINB_32_memop;
    stsminh(rs: Register32, rn: Register64OrSp) => STSMINH_LDSMINH_32_memop;
    stsminl_32(rs: Register32, rn: Register64OrSp) => STSMINL_LDSMINL_32_memop;
    stsminl_64(rs: Register64, rn: Register64OrSp) => STSMINL_LDSMINL_64_memop;
    stsminlb(rs: Register32, rn: Register64OrSp) => STSMINLB_LDSMINLB_32_memop;
    stsminlh(rs: Register32, rn: Register64OrSp) => STSMINLH_LDSMINLH_32_memop;
    stumax_32(rs: Register32, rn: Register64OrSp) => STUMAX_LDUMAX_32_memop;
    stumax_64(rs: Register64, rn: Register64OrSp) => STUMAX_LDUMAX_64_memop;
    stumaxb(rs: Register32, rn: Register64OrSp) => STUMAXB_LDUMAXB_32_memop;
    stumaxh(rs: Register32, rn: Register64OrSp) => STUMAXH_LDUMAXH_32_memop;
    stumaxl_32(rs: Register32, rn: Register64OrSp) => STUMAXL_LDUMAXL_32_memop;
    stumaxl_64(rs: Register64, rn: Register64OrSp) => STUMAXL_LDUMAXL_64_memop;
    stumaxlb(rs: Register32, rn: Register64OrSp) => STUMAXLB_LDUMAXLB_32_memop;
    stumaxlh(rs: Register32, rn: Register64OrSp) => STUMAXLH_LDUMAXLH_32_memop;
    stumin_32(rs: Register32, rn: Register64OrSp) => STUMIN_LDUMIN_32_memop;
    stumin_64(rs: Register64, rn: Register64OrSp) => STUMIN_LDUMIN_64_memop;
    stuminb(rs: Register32, rn: Register64OrSp) => STUMINB_LDUMINB_32_memop;
    stuminh(rs: Register32, rn: Register64OrSp) => STUMINH_LDUMINH_32_memop;
    stuminl_32(rs: Register32, rn: Register64OrSp) => STUMINL_LDUMINL_32_memop;
    stuminl_64(rs: Register64, rn: Register64OrSp) => STUMINL_LDUMINL_64_memop;
    stuminlb(rs: Register32, rn: Register64OrSp) => STUMINLB_LDUMINLB_32_memop;
    stuminlh(rs: Register32, rn: Register64OrSp) => STUMINLH_LDUMINLH_32_memop;
    swp_32(rs: Register32, rt: Register32, rn: Register64OrSp) => SWP_32_memop;
    swp_64(rs: Register64, rt: Register64, rn: Register64OrSp) => SWP_64_memop;
    swpa_32(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPA_32_memop;
    swpa_64(rs: Register64, rt: Register64, rn: Register64OrSp) => SWPA_64_memop;
    swpab(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPAB_32_memop;
    swpah(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPAH_32_memop;
    swpal_32(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPAL_32_memop;
    swpal_64(rs: Register64, rt: Register64, rn: Register64OrSp) => SWPAL_64_memop;
    swpalb(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPALB_32_memop;
    swpalh(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPALH_32_memop;
    swpb(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPB_32_memop;
    swph(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPH_32_memop;
    swpl_32(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPL_32_memop;
    swpl_64(rs: Register64, rt: Register64, rn: Register64OrSp) => SWPL_64_memop;
    swplb(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPLB_32_memop;
    swplh(rs: Register32, rt: Register32, rn: Register64OrSp) => SWPLH_32_memop;
}
