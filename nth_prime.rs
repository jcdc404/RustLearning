
// fn c_to_f(x:f32)->f32{
//     (x*9.0/5.0)+32.0
// }
// fn f_to_c(x:f32)->f32{
//     ((x-32.0)*5.0/9.0)
// }
// fn nth_fib(x: i64){
//     let mut a: i64 = 1;
//     let mut b: i64 = 1;
//     let mut fib: i64 = 0;
//     for _i in 0..(x-2) {
//         fib=a+b;
//         a=b;
//         b=fib;
//     }
//     println!("{}",fib)
// }

//fn find_nth_prime(x:i32)->i32{
//	let mut primes = vec![2,3];
//	let mut len_primes: i32 = 2;
//	let mut i = 5;
//	
//	while len_primes<x{
//		for j in &primes{
//		    if i%j==0{
//				break;
//			}else if j*j>i{
//				primes.push(i);
//				len_primes+=1;
//				break;
//			}
//		}
//		i+=2;	
//	}
//	i-2
//}

//     println!("The prime you're looking for is {}",found[found.len()-1])
// }

// scalar types: i/u:8/16/32/64/128/size
//              bool: true, false
//              char (4bytes)
//              f32/64
//              array: fixed length at assignment of a homogenous type, contiguous chunk of memory "let arr: [i32; 5] = [1,2,3,4,5]"
//              tuple: fixed length at assignment of a heterogenous type "let tup: (i32, f64, u8) = (500, 6.4, 1)"

// operators: all the standard ones +,-,*,/,% 
//                    combo         +=,-=,*=,/=,%= 
//                    comparison    <,>,=,!=, <=, >=
//                    bitwise       <<,>>,|,^,&,! // left shift, right shift, OR, XOR, AND, NOT 

// functions "fn a_function(){ code..goes..here}"
//     with params "fn another_function(x: i32, y: f64){ code..goes..here}"
//     with return values "fn five_times(x: i32)->i32{x*5}"//this would return the input times five in i32 format

// statement vs expression: 
//      expression returns a value
//      statement represents and action or command

// Control Flow:
//  if: the condition MUST be a bool
//  else if, else -- the remaining branching keywords
//  let number = if x==5{ 12 } else { 15 }; -- this would evaluate number to 12 if x is 5 otherwise numbert would be 15

//  loop - essentially a while true, flow is controlled with continue and break
//      a return value can be returned from the break
//          loop {counter +=1; if counter ==15{break 5}}

//  while - provides a conditioned loop
//      while counter<30{counter += 1;} // this is evaluated before running, so if counter initializes at 31, it never runs

//  for - a controlled loop allowing an iterator/range
//    WITH ITERATOR
//      let a: [i32;5] = [1,2,3,4,5];
//      for i in a.iter(){
//          println!("{}",i)
//      }
//    WITH EXCLUSIVE RANGE
//      for i in 1..5 {println!("{}",i)}
//    WITH INCLUSIVE RANGE
//      for i in 1..=5 {println!("{}",i)}
//

// String type -- stored on the heap

fn main() {
	let s="hello";
	let s1="hello1";
	let s2="hello2";
	let hello_vec = vec!(s,s1,s2);
for i in &hello_vec{
	println!("{}",i);
}


}