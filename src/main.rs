//gonna rewrite everything here,i have think for this quite a long time...i kinda suck so rewrite it then
//well why think such that much isnt it,just write sth out is fine bruh and then immediately go start my new project then
/* i have think for quite a long time like why not...make it chaotic?
	print x # it is now in a stage of holding?
	let x be y 
	let y be "sigma" #like print it out here because now it is defined?
	//or like...
	print let x be loop{
		fahhh+=1
		let fahhh be x
		let x be 3
		if fahhh == 3 {return}{fahhh+1}
	}
	//urr upper one just dont bruh kindasuckbruh
	oh yeah i got idea,why not like lekker first see what can assign and if syntax error just skip that line bruh like if need see error then on first line say:give me error
	then i suppose it wont need traditional lexer and parser lmao much easier and i can even write function one by one
	i have plan now:
	1.lekker will auto skip all error lines and try to execute codes that is still normal to use so you need to be syntax perfect and logic perfect but at the same time you dont need afraid of tons of error anymore
	yeah i finalely used my brain,if now rules is make by me why dont make it whatever i want lmao
*/
//project redefine here and let restrict must use \" as String
//first write the input and output and variable assigning
//1.output 2.variable assigning 3. input
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
enum variables{
	string(String),
	numbers(f64),
	bl(bool),//True and False to represent then
	list(Vec<variables>),
	Null,
}
struct lekker{
	functions:HashMap<String,String>,//here for using set to make function which call the sub-program PS:for future usage
	variables:HashMap<String,variables>,
	code:String,
}
impl lekker{
	fn sbyblock(&self)->Vec<String>{//still maybe just lower the level to split by blocks as first init split
		let mut temp:String=String::new();
		let mut temp_list:Vec<String>=Vec::new();
		//i think my chinese need retest bruh holly
		let codes=self.code.clone();
		let codes=codes.clone();
		let codes=codes.split("").collect::<Vec<_>>();
		let mut wnum:i32=0;
		let mut instr:bool=false;
		let mut is_comment:bool=false;
		for i in 0..codes.len(){
			let value=codes[i];
			if wnum>0{
				match value{
					"{"=>{if !(instr||is_comment){wnum+=1;}},
					"}"=>{if !(instr||is_comment){wnum-=1;}},
					"\""=>{instr= !instr;},
					"#"=>{is_comment= !is_comment;},
					_=>{},
				}
				temp.push_str(value);
				if i+1<codes.len(){continue}
				else{temp_list.push(temp.clone());break}
			}else if wnum<0{
				temp_list.push(temp.clone());
				break
			}else{
				
				match value{
					"{"=>{if !(instr||is_comment){wnum+=1;}},
					"}"=>{if !(instr||is_comment){wnum-=1;}},
					"\""=>{instr= !instr;},
					"#"=>{is_comment= !is_comment;},
					_=>{},
				}
				temp.push_str(value);
				match value{
					"\n"=>{temp_list.push(temp.clone());temp=String::new();},
					_=>{},
				}
				if i+1<codes.len(){continue}
				else{temp_list.push(temp.clone());break}
			}
		}
		temp_list
	}//done the first spliting
	fn sbyspace(&self,bruh:String)->Vec<String>{//now second step to split by space
		//well now i am start thinking like if you meet {blablabla} and how?
		//chinese test really cooked
		//so then just make this become a small func instead of a big func then bruh
		let codes=bruh.clone();
		let codes=codes.clone();//make it long lasting
		let codes=codes.split("").collect::<Vec<_>>();
		//uhh then? fine write this tmr then i need to do hw bruh
		//i still want write code instead of doing hw bruh
		let mut temp:String=String::new();
		let mut temp_list:Vec<String>=Vec::new();
		let mut is_comment:bool=false;
		let mut instr:bool=false;
		let mut wnum:i32=0;
		for i in 0..codes.len(){
			let value=codes[i];
			if wnum>0{
				match value{
					"{"=>{if !(instr||is_comment){wnum+=1;}},
					"}"=>{if !(instr||is_comment){wnum-=1;}},
					"\""=>{instr= !instr;},
					"#"=>{is_comment= !is_comment;},
					_=>{},
				}
				match value{
					"}"=>{if wnum>0&& !is_comment{temp.push_str(value);}},
					"#"=>{},
					_=>{if !is_comment{temp.push_str(value);}},
				}
				if i+1<codes.len(){continue}
				else{temp_list.push(temp.clone());break}
			}else if wnum<0{
				temp_list.push(temp.clone());
				break
			}else{
				match value{
					"{"=>{if !(instr||is_comment){wnum+=1;}},
					"}"=>{if !(instr||is_comment){wnum-=1;}},
					"\""=>{instr= !instr;},
					"#"=>{is_comment= !is_comment;},
					_=>{},
				}
				match value{
					"\n"=>{},
					"{"=>{},
					"#"=>{},
					" "=>{
						if !temp.is_empty()&& !(instr||is_comment){
							temp_list.push(temp.clone());
							temp=String::new();
						}
						else{if !is_comment{temp.push_str(value);}}
					},
					_=>{if !is_comment{temp.push_str(value);}},
				}
				if i+1<codes.len(){continue}
				else{temp_list.push(temp.clone());break}
			}
		}
		temp_list				
	}//done lmao next step is to do the separate by comma for list and print use
	fn exec(&self){//uhh write this after the comma separation
		//after a little bit of considering  i might just embed it to different which is assign and print then
		//call the print function to perform printing(first function)
	}
	fn lsbycomma(&self,printwhat:String)->Vec<String>{//varname still printwhat but actually function normally,ignore the variable name
		let printer:String=printwhat.clone();
		let printer=printer.clone();
		let printer=printer.split("").collect::<Vec<_>>();
		let mut temp:String=String::new();
		let mut temp_list:Vec<String>=Vec::new();
		let mut instr:bool=false;
		let mut lbnum:i32=0;
		for i in 0..printer.len(){
			let value=printer[i];
			if lbnum>0{
				match value{//here we can ignore if have like {} sth like that cuz get_value will deal with it
					"["=>{if !instr{lbnum+=1;}},
					"]"=>{if !instr{lbnum-=1;}},
					"\""=>{instr= !instr;},//chinese retest :(
					_=>{},
				}//here can refer to sbyblock for idea
				if lbnum>0{temp.push_str(value);}
				if i+1<printer.len(){continue}
				else{temp_list.push(temp.clone());break}
			}else if lbnum<0{//dont overthinking
				temp_list.push(temp.clone());
				break
			}else{
				match value{
					"["=>{if !instr{lbnum+=1;}},
					"]"=>{if !instr{break}},
					"\""=>{instr= !instr;},
					","=>{
						if !instr&&!temp.is_empty(){
							temp_list.push(temp.clone());
							temp=String::new();
						}else{temp.push_str(value);}
					},
					_=>{temp.push_str(value);},
				}
				if i+1<printer.len(){continue}
				else{
					if !temp.is_empty(){temp_list.push(temp.clone());}
					break
				}
			}				
		}//here done the comma separation
		temp_list
		//here gonna separate this as my original target which is lsbycomma<-little bit modify cuz it mainly target on list actually
	}
	fn get_value(&self,get:String)->variables{
		variables::Null
	}
}//actually i can wrap it inside itself when encounter loop bruh

	
fn main(){
	let path:Vec<_>=env::args().collect();
	if path.len()!=2{
		eprint!("Error:Arguments too much or not enough,suppose should be ./lekker [your file] || lekker.exe [your file]");
		exit(1)
	}else{
	/*
	functions:HashMap<String,String>,
	variables:HashMap<String,variables>,
	code:String,
	*/
		let content:String=fs::read_to_string(path[1].clone()).unwrap().to_string();
		let mut smain=lekker{
			functions:HashMap::new(),
			variables:HashMap::new(),
			code:content.clone(),
		};
		let tester=smain.sbyblock();
		println!("init split:{:?}\n",tester.clone());
		for i in tester.clone(){
			println!("second split:{:?}",smain.sbyspace(i));
		}
		println!("comma split:{:?}",smain.lsbycomma(smain.sbyspace(tester[0].clone())[1].clone()));
	}	
}
