extern crate curve25519_dalek;
extern crate rand_core;
extern crate colored;

use rand_core::OsRng;
use curve25519_dalek::traits::MultiscalarMul;
use curve25519_dalek::{constants, ristretto::RistrettoPoint, scalar::Scalar};
use crate::test::Student;


pub struct Commitment(pub RistrettoPoint); 
pub struct CommitmentOpening(pub Scalar); 

#[derive(Clone)]
pub struct VerifierPublicKey(pub RistrettoPoint); 
#[derive(Debug)]
pub struct CommitmentValue(pub Scalar); 
pub struct Committer; 

pub struct CommitVerifier {  
    pub pk: VerifierPublicKey,
    pub commitment: Option<Commitment>, 
}


impl CommitmentValue {  
    pub fn from_u64(x: u64) -> Self {
        CommitmentValue(Scalar::from(x))
    }
    
}

impl CommitVerifier {
    pub fn init(mut rng: Scalar) -> (VerifierPublicKey, Self) {
       // let a: Scalar = Scalar::random(&mut rng);
        let G = &constants::RISTRETTO_BASEPOINT_POINT;
        let H = rng * G;
        let pub_key = VerifierPublicKey(H);
        (
            pub_key.clone(),
            CommitVerifier {
                pk: pub_key,
                commitment: None,
            },
        )
    }

    pub fn receive_commitment(&mut self, commitment: Commitment) {
        self.commitment = Some(commitment);
    }

    pub fn verify(&self, val: &CommitmentValue, commitment_opening: &CommitmentOpening) -> bool {
        if let Some(Commitment(C)) = self.commitment {
            let VerifierPublicKey(H) = self.pk;
            let G = &constants::RISTRETTO_BASEPOINT_POINT;
            let &CommitmentOpening(r) = commitment_opening;
            let &CommitmentValue(m) = val;
            
            let C2 = RistrettoPoint::multiscalar_mul(vec![r, m], vec![G, &H]);
           // println!("{:?},{:?},{:?},{:?}",C,r,m,C2);
          // println!("C2: {:?}",C2);
            C == C2
        } else {
            panic!("No commitment received");
        }
    }
    pub fn verify_grades(&self, val: &CommitmentValue,) {

    }
}

impl Committer {
    pub fn commit(mut rng: &mut OsRng, val: &CommitmentValue, pk: &VerifierPublicKey)
                          -> (RistrettoPoint,Commitment, CommitmentOpening) {
        let r = Scalar::random(&mut rng);
        let &CommitmentValue(val_as_scalar) = val;
        let G = &constants::RISTRETTO_BASEPOINT_POINT;
        let &VerifierPublicKey(pub_key_point) = pk;
        let C = RistrettoPoint::multiscalar_mul(&[r, val_as_scalar], [G, &pub_key_point]);
        (C,Commitment(C), CommitmentOpening(r))
    }
    pub fn add_commit(mut openingvec : Vec<CommitmentOpening>, mut rng: Scalar, mut val: &Student)-> (Commitment,CommitmentOpening){
        let G=&constants::RISTRETTO_BASEPOINT_POINT;
        let H= rng*G;
        let CommitmentOpening(opening1)= openingvec[0];
        let CommitmentOpening(opening2)= openingvec[1];
        let CommitmentOpening(opening3)= openingvec[2];
        let CommitmentOpening(opening4)= openingvec[3];
        let opening=opening1+opening2+opening3+opening4;
        let CommitmentValue(score1) = CommitmentValue::from_u64(val.score1);
        let CommitmentValue(score2) = CommitmentValue::from_u64(val.score2);
        let CommitmentValue(score3) = CommitmentValue::from_u64(val.score3);
        let CommitmentValue(score4)= CommitmentValue::from_u64(val.score4);
        let score=score1+score2+score3+score4;
        //println!("1: {:?} 2: {:?} 3: {:?} 4: {:?} 5: {:?}", score,score1,score2,score3,score4);
        let C= RistrettoPoint::multiscalar_mul(vec![opening,score],vec![G,&H]);
       // println!("C: {:?}",C);
        (Commitment(C),CommitmentOpening(opening))
    }
}