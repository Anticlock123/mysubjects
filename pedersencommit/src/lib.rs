pub mod commitment;
pub mod test;
use test::*;
use commitment::*;
use office::Excel;
use std::collections::HashMap;
use colored::*;
extern crate curve25519_dalek;
extern crate rand_core;
extern crate colored;

use rand_core::OsRng;
use curve25519_dalek::{constants,scalar::Scalar};



pub fn test_pedersen() {
//    // let true_result:bool = test::good_test();
//    // println!("True Result: {}",true_result);
//    // let bad_opening_result:bool = bad_opening_test();
//    // println!("Bad opening Result: {}",bad_opening_result);
//    // let bad_commitment_result:bool = bad_commitment();
//    // println!("Bad commitment Result: {}",bad_commitment_result);
//     let mut excel = Excel::open("grades.xlsx").unwrap();
//     let r = excel.worksheet_range("Sheet1").unwrap();
//     let mut grades_data  = r.rows().collect::<Vec<_>>();
//     let mut count = 0;
//     let mut name : String= String::new();
//     let mut gradesnum: HashMap<String,Vec<f64>> = HashMap::new();
//     for grade_itor in grades_data.iter() {
//         if count == 0 {
//             count += 1;
//             continue;
//         }
//         let mut grades: Vec<f64> = Vec::new();

//         for items in grade_itor.iter() {
//             match &items{
//                 office::DataType::String(v) =>{
//                   name= v.clone();
//                 },
//                 office::DataType::Float(v)=>{
//                     let mut temp : f64= 100.0;
//                     temp= v.clone();
//                     grades.push(temp);
//                 },
//                 _=>(),
//             };
//         }
        
//         gradesnum.insert(name.to_string(),grades);
//     }
//     for (key,mut value) in gradesnum{
//         let mut rng= OsRng;
//         let a : Scalar = Scalar::random(&mut rng);
//         println!("The verifier sends a random scalar {:?} to the prover.",a);

//         println!("Prover is {} and Start uploading the scores of each subject...",key.yellow());
//         let mut valueqz :Vec<u64>= Vec::new();
        
//         let length :usize = value.len();
//        // println!("{}",length);
//         for x in 0..length{
//             valueqz.push((value[x]*100.0) as u64);
//         }
        
//         let (subject1,opening1,verify1)=grades_test(valueqz[0]);
//         println!("{} uploaded successfully. Pedersen Commitment Value is {:?}","Subject 1".blue(),subject1);
//         let (subject2,opening2,verify2)=grades_test(valueqz[1]);
//         println!("{} uploaded successfully. Pedersen Commitment Value is {:?}","Subject 2".blue(),subject2);
//         let (subject3,opening3,verify3)=grades_test(valueqz[2]);
//         println!("{} uploaded successfully. Pedersen Commitment Value is {:?}","Subject 3".blue(),subject3);
//         let (subject4,opening4,verify4)=grades_test(valueqz[3]);
//         println!("{} uploaded successfully. Pedersen Commitment Value is {:?}","Subject 4".blue(),subject4);

//         println!("The prover has completed his commitment...");
//         println!("The prover informs the verifier of its average score is {}...",(valueqz[4] as f64/100.0).to_string().yellow());

//         let mut commit_opening : Vec<CommitmentOpening>= Vec::new();
//         commit_opening.push(opening1);
//         commit_opening.push(opening2);
//         commit_opening.push(opening3);
//         commit_opening.push(opening4);

//         let  (prover,ropening )= Committer::add_commit(commit_opening,a,&valueqz);
        
//         let sum = valueqz[valueqz.len()-1]*4;
        
//         //println!("The sum:{}",sum);
//          let averageval = CommitmentValue::from_u64(sum);
//         let G= &constants::RISTRETTO_BASEPOINT_POINT;
//         let mut verifier= CommitVerifier{
//             pk : VerifierPublicKey(a*G),
//             commitment: None,
//         };
//         verifier.receive_commitment(prover);
//         let flag:bool = verifier.verify(&averageval,&ropening);
//         print!("The verifier validates the result is ");
//         match flag {
//             ture =>println!("{}!","True".italic().green()),
//             false => println!("{}!","False".red()),
//         };
//         println!("{}","----------------------------------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------------------------------".blue());
//     }


}
