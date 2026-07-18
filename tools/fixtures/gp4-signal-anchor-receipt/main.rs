use derived_world_rules::{compile_world, CausalWorldPacket, WorldGenerationInput};
use mindwarp_gameplay_foundation::fixed_sessions;
use mindwarp_signal_anchor_vertical::build_signal_anchor_bundle;
use mindwarp_vertical_persistence::VerticalLogV1;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{env, fs};

mod world_support {
    include!(concat!(env!("FORGE_ROOT"), "/crates/mindwarp-gameplay-foundation/tests/world_support/mod.rs"));
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DependencyReceiptV1 { id:String, proof_kind:String, proof_ref:String }

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct CloseoutReceiptV1 {
    schema_version:u16, receipt_id:String, outcome:String, bundle_id:String,
    bundle_schema_version:u16, registered_full_gate:String, registered_checkpoint_sha256:String, registered_duration_ms:u64,
    bundle_digest:String, bundle_byte_sha256:String, bundle_byte_length:u64, final_head:String,
    dependency_receipts:Vec<DependencyReceiptV1>, broad_g1:bool, runtime_selected:bool,
    runtime_containment_pending:bool, evidence_only:bool, promotion_authority:bool,
    lifecycle_state:String, lifecycle_status:String, receipt_hash:String,
}

fn hex(bytes:impl AsRef<[u8]>)->String{bytes.as_ref().iter().map(|b|format!("{b:02x}")).collect()}
fn expected()->CloseoutReceiptV1 {
    let input:WorldGenerationInput=world_support::world_input([0x4a;32]); let packet:CausalWorldPacket=compile_world(&input).unwrap();
    let bundle=build_signal_anchor_bundle(&input,&packet).unwrap(); let bytes=bundle.to_bytes().unwrap();
    let record=fixed_sessions().into_iter().find(|r|r.session_id=="gp0.s4.signal-anchor").unwrap();
    let log=VerticalLogV1::from_bytes(&bundle.c4v_log_bytes,&record,&input,&packet).unwrap();
    let mut value=CloseoutReceiptV1{schema_version:1,receipt_id:"G1-VERTICAL-CLOSEOUT".into(),outcome:"passed".into(),bundle_id:"gp4.signal-anchor.bundle-v1".into(),bundle_schema_version:1,registered_full_gate:"run-7e5c44dc8f48424a8cec42da756e3127".into(),registered_checkpoint_sha256:"8427844116d40de75119565aadd056182c062273262c78d7fa509c3b7f47b93c".into(),registered_duration_ms:590582,bundle_digest:hex(bundle.bundle_digest),bundle_byte_sha256:hex(Sha256::digest(&bytes)),bundle_byte_length:bytes.len()as u64,final_head:hex(log.head().unwrap().unwrap()),dependency_receipts:vec![
      DependencyReceiptV1{id:"C3A".into(),proof_kind:"c3a-fixed-input-packet-v1".into(),proof_ref:"input-sha256:5f54137fa9de4b06514dbfde509ef5faf65a23b885a24288ed5cb51bbcee07ca;packet-id:947a0564c7a08115d4ee63ff89bfbdafdc9303ecd7f86c846b4945c7e305492b;packet-sha256:e3479b36a3e7085ae892a358ba7e5e6415688ef0d82e0338b9226ae71c46576f".into()},
      DependencyReceiptV1{id:"C4V".into(),proof_kind:"registered-full-gate-v1".into(),proof_ref:"run-fa6334a300e04d409dd5cddb4f22542e".into()},
      DependencyReceiptV1{id:"GP0".into(),proof_kind:"registered-full-gate-v1".into(),proof_ref:"run-79fca2c134994b76a119aebc0987a4fd".into()},
      DependencyReceiptV1{id:"GP1".into(),proof_kind:"registered-full-gate-v1".into(),proof_ref:"run-26b8424e488b4d838eb51cc928675224".into()},
      DependencyReceiptV1{id:"GP2".into(),proof_kind:"registered-full-gate-v1".into(),proof_ref:"run-2dc3db644adc416a8ef56461dbb771b6".into()},
      DependencyReceiptV1{id:"GP3".into(),proof_kind:"registered-full-gate-v1".into(),proof_ref:"run-50a8c78043eb46c483f1f655d3793f9b".into()},
      DependencyReceiptV1{id:"GP4".into(),proof_kind:"registered-full-gate-v1".into(),proof_ref:"run-7e5c44dc8f48424a8cec42da756e3127".into()}],broad_g1:false,runtime_selected:false,runtime_containment_pending:true,evidence_only:true,promotion_authority:false,lifecycle_state:"executing".into(),lifecycle_status:"active".into(),receipt_hash:String::new()};
    value.receipt_hash=receipt_hash(&value); value
}
// Hash framing is domain || NUL || big-endian u64 byte length || canonical JSON
// with receipt_hash set to the empty string. This prevents concatenation ambiguity.
fn receipt_hash(value:&CloseoutReceiptV1)->String{let mut copy=value.clone();copy.receipt_hash=String::new();let body=serde_json::to_vec(&copy).unwrap();let mut h=Sha256::new();h.update(b"mindwarp.gp4.vertical-closeout.receipt.v1\0");h.update((body.len()as u64).to_be_bytes());h.update(body);hex(h.finalize())}
fn validate(bytes:&[u8])->Result<CloseoutReceiptV1,()>{let value:CloseoutReceiptV1=serde_json::from_slice(bytes).map_err(|_|())?;let mut canonical=serde_json::to_vec(&value).map_err(|_|())?;canonical.push(b'\n');if canonical!=bytes||value!=expected()||receipt_hash(&value)!=value.receipt_hash{return Err(())}Ok(value)}
fn main(){let expected=expected();if env::var_os("COMPUTE_ONLY").is_some(){println!("{}",serde_json::to_string(&expected).unwrap());return}let path=env::var("RECEIPT_PATH").unwrap();let bytes=fs::read(path).unwrap();let value=validate(&bytes).unwrap();
  for i in 0..17{let mut v=value.clone();match i{0=>v.schema_version=2,1=>v.registered_full_gate.push('x'),2=>v.registered_checkpoint_sha256.push('x'),3=>v.registered_duration_ms+=1,4=>v.bundle_digest.push('x'),5=>v.bundle_byte_sha256.push('x'),6=>v.bundle_byte_length+=1,7=>v.final_head.push('x'),8=>v.dependency_receipts.swap(0,1),9=>v.dependency_receipts[0].proof_ref.push('x'),10=>v.broad_g1=true,11=>v.runtime_selected=true,12=>v.runtime_containment_pending=false,13=>v.evidence_only=false,14=>v.promotion_authority=true,15=>v.lifecycle_state="complete".into(),_=>v.receipt_hash.push('x')}let mut hostile=serde_json::to_vec(&v).unwrap();hostile.push(b'\n');if validate(&hostile).is_ok(){panic!("hostile {i} admitted")}}
  let text=String::from_utf8(bytes).unwrap();for hostile in [text.replacen("{","{\"unknown\":true,",1),text.replacen("\"broad_g1\":false,","",1),text.replacen("\"registered_duration_ms\":590582","\"registered_duration_ms\":\"590582\"",1),text.replacen("\"broad_g1\":false","\"broad_g1\":\"false\"",1)]{assert!(validate(hostile.as_bytes()).is_err())}
  println!("G1 vertical closeout receipt verified: {} bytes, deterministic framed hash, exact ordered proofs and authority/lifecycle hostiles fail closed.",value.bundle_byte_length);
}
