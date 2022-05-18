use super::commitment::*;
extern crate curve25519_dalek;
extern crate rand_core;
extern crate colored;

use rand_core::OsRng;
use curve25519_dalek::{constants, ristretto::RistrettoPoint, scalar::Scalar};

pub struct Student {
    pub id: i32,
    pub name: String,
    pub score1: u64,
    pub score2: u64,
    pub score3: u64,
    pub score4: u64,
    pub score5: u64,
    pub key: String,
}

pub fn get_rng_point() -> String {
    let mut rng= OsRng;
    let a: Scalar = Scalar::random(&mut rng);
    //println!("{:?}", a);
    let b = a.to_bytes();
    let c: Vec<u8> = b.to_vec();
    let mut string2 = String::new();
    let mut index = 0;
    for x in c.iter() {
        let s = x.to_string();
        if index == 0 {
            string2 += &s;
        }else {
            string2 += ",";
            string2 += &s;
        }
        index += 1;
    }
    string2
    

}
pub fn string_to_scalar(var: String) -> Scalar {
    let v: Vec<&str>= var.split(",").collect();
    let mut pp : [u8;32]= [0;32];
    let mut index = 0;
    for x in v.iter() {
        let r= x.to_string();
        pp[index]=r.parse::<u8>().unwrap();
        index += 1;
    }
    let a : Scalar =Scalar::from_bytes_mod_order(pp);
    a

}
pub fn pover_and_verify(student: &Student,a: Scalar) -> bool {

        let (subject1,opening1,verify1)=grades_test(student.score1);
        let (subject2,opening2,verify2)=grades_test(student.score2);
        let (subject3,opening3,verify3)=grades_test(student.score3);
        let (subject4,opening4,verify4)=grades_test(student.score4);


        let mut commit_opening : Vec<CommitmentOpening>= Vec::new();
        commit_opening.push(opening1);
        commit_opening.push(opening2);
        commit_opening.push(opening3);
        commit_opening.push(opening4);

        let  (prover,ropening )= Committer::add_commit(commit_opening,a,student);
        
        let sum = student.score5;
        
        //println!("The sum:{}",sum);
         let averageval = CommitmentValue::from_u64(sum);
        let G= &constants::RISTRETTO_BASEPOINT_POINT;
        let mut verifier= CommitVerifier{
            pk : VerifierPublicKey(a*G),
            commitment: None,
        };
        verifier.receive_commitment(prover);
        let flag:bool = verifier.verify(&averageval,&ropening);
        print!("The verifier validates the result is ");
        match flag {
            ture => {true},
            false => {false}
        }
}
pub fn pedersen_power(student: &Student, pk: String, bad_var: u64) ->bool{
    let mut rng= OsRng;
    let a : Scalar = string_to_scalar(pk);

    let val = CommitmentValue::from_u64(student.score5);

    let (verifier_pub_key, mut verifier) = CommitVerifier::init(a);
    let bad_val = CommitmentValue::from_u64(bad_var);
    let (commitmentvalue,_commitment, commitment_opening) = Committer::commit(&mut rng, &val, &verifier_pub_key);
    let (_, bad_commitment,_ )= Committer::commit(&mut rng, &bad_val, &verifier_pub_key);
    // let bad_commitment = Commitment(Scalar::from(bad_var) * constants::RISTRETTO_BASEPOINT_POINT);
    verifier.receive_commitment(bad_commitment);

    let bad_verify = verifier.verify(&val, &commitment_opening);

    bad_verify == true
    
}
pub fn pedersen_test(one_student: Student, two_student: Student) -> bool {
    let a1= string_to_scalar(one_student.key.clone());
    let a2= string_to_scalar(two_student.key.clone());

    let flag_one = pover_and_verify(&one_student, a2);
    let flag_two = pover_and_verify(&two_student, a1);
    if flag_one && flag_two {
        true
    }else {
        false
    }


}


pub fn bad_opening_test() -> bool{
        let mut rng= OsRng;
        let a : Scalar = Scalar::random(&mut rng);
      
        let val = CommitmentValue::from_u64(3);

        let (verifier_pub_key, mut verifier) = CommitVerifier::init(a);
        let (commitmentvalue,commitment, _commitment_opening) = Committer::commit(&mut rng, &val, &verifier_pub_key);

        verifier.receive_commitment(commitment);

        let bad_commitment_opening = CommitmentOpening(Scalar::from(4u64));
        let bad_verify = verifier.verify(&val, &bad_commitment_opening);

        bad_verify == true
}

pub fn bad_commitment() -> bool {
    let mut rng= OsRng;
    let a : Scalar = Scalar::random(&mut rng);

    let val = CommitmentValue::from_u64(3);

    let (verifier_pub_key, mut verifier) = CommitVerifier::init(a);
    let (commitmentvalue,_commitment, commitment_opening) = Committer::commit(&mut rng, &val, &verifier_pub_key);

    let bad_commitment = Commitment(Scalar::from(2u64) * constants::RISTRETTO_BASEPOINT_POINT);
    verifier.receive_commitment(bad_commitment);

    let bad_verify = verifier.verify(&val, &commitment_opening);

    bad_verify == true
}

pub fn grades_test(score: u64) -> (RistrettoPoint,CommitmentOpening,CommitVerifier){
    let mut rng= OsRng;
    let a : Scalar = Scalar::random(&mut rng);
    let val = CommitmentValue::from_u64(score);
    let (verifier_pub_key, mut verifier) = CommitVerifier::init(a);
    let (commitmentvalue,commitment, commitment_opening) = Committer::commit(&mut rng, &val, &verifier_pub_key);
    (commitmentvalue,commitment_opening,verifier)
}