//lekker is a programming language(percise as interpreter) built based on rust (before it can bootstrap itself)
/*expect syntax(current version):
	#x{12.9*31}; //# for setting the value(all will be mutable) of name between # and < inside <> and end with;
	!print{"12.9*31="x}; //function between ! and < are decide for what to do <> and abandoning the , also can do @x if need call gobal value(set outside loop)
	#y{!input{}}; //well you can do this
	#z{["sigma"&x&[y&12.3]]};
	//also deciding the and or not also () become:!and{condition1,condition2}...you got it and also the comparasion stuff,all embed to function(like !LOET{condition1,condition2} which represented larger or equal to) and the bracket become !{} also the if else also embeded to function and also loop will not count indentation but curly bracket number like !loop{}
	also somehow forgot but will use $ as setting a function/module
*/
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
#[derive(Debug)]
enum types{
	end,
	function,
	maths,
	number,
	string,
	boolean,//^ || ~ to represent
	slist,
	elist,
	scurly,
	ecurly,
	variable,
	separater,//the &
}
enum variable_type{
	number(f64),
	string(String),
	boolean(bool),
	list(Vec<variable_type>),
}
enum fncall{
	print,
	input,
	equal,
	lt,//larger than
	st,//smaller than
	not,
	and,
	or,
	bracket,//for performing () if it is !{}
	lloop,//l for lekker
	lif,//l for lekker
	lelse,//l for lekker
	intolist,//separating "" so it become a list
	fsread,
	fswrite,//both for future bruh
}
enum func{
	function(fncall),//perform the !
	pullfromglobal,//perform the @
	assign,//perform the #
	fnset,//perform the $
}
enum everything{
	fnc(func),
	var(variable_type),
	ltype(types),
}
//extract all into char and label with different types
fn lexer(codet:String)->(Vec<types>,Vec<String>){
	let mut chartype:Vec<types>=Vec::new();
	let mut chartemp:Vec<String>=Vec::new();
	let mut iibk:bool=false;
	let mut ibk:bool=false;
	for i in codet.split("").collect::<Vec<_>>(){
		if iibk||ibk{
			if !ibk&&i=="\'"{iibk=!iibk;}
			else if !iibk&&i=="\""{ibk=!ibk;}
			chartype.push(types::string);
			chartemp.push(i.to_string());
		}else{
			if i=="\""{ibk=!ibk;chartype.push(types::string);chartemp.push(i.to_string());}
			else if i=="\'"{iibk=!iibk;chartype.push(types::string);chartemp.push(i.to_string());}
			else if !" \t\n".contains(&i){
				if "1234567890.".contains(&i){chartype.push(types::number);chartemp.push(i.to_string());}
				else if "+=-*/%".contains(&i){chartype.push(types::maths);chartemp.push(i.to_string());}
				else if "^~".contains(&i){chartype.push(types::boolean);chartemp.push(i.to_string());}
				else if "!#@$".contains(&i){chartype.push(types::function);chartemp.push(i.to_string());}
				else if i==";"{chartype.push(types::end);chartemp.push(i.to_string());}
				else if i=="["{chartype.push(types::slist);chartemp.push(i.to_string());}
				else if i=="]"{chartype.push(types::elist);chartemp.push(i.to_string());}
				else if i=="{"{chartype.push(types::scurly);chartemp.push(i.to_string());}
				else if i=="}"{chartype.push(types::ecurly);chartemp.push(i.to_string());}
				else if i=="&"{chartype.push(types::separater);chartemp.push(i.to_string());}
				else{chartype.push(types::variable);chartemp.push(i.to_string());}
			}
		}
	}
	(chartype,chartemp)
}
//recieve the result from lexer and start check the grammar also merge the char and types into one Vec<variable_type>
//remember to create a whole new enum for this like enum store everything so it can execute it i guess
fn parser(bruh:(Vec<types>,Vec<String>)){}

struct lekker{
	codebyblock:Vec<everything>,//also change this cuz this struct only do execution like become Vec<??>
	global_variable:HashMap<String,variable_type>,
	temp:Vec<HashMap<String,variable_type>>,
	local_variable:HashMap<String,variable_type>,
	current_block:usize
}
fn main(){
	let path:Vec<_>=env::args().collect();
	if path.len()!=2{
		eprint!("Error:Arguments too much or not enough,suppose should be ./lekker [your file] || lekker.exe [your file]");
		exit(1)
	}else{
		let content:String=fs::read_to_string(path[1].clone()).unwrap().to_string();
		let (a,b)=lexer(content);
		println!("code lexed:{:?}\nraw:{:?}",a,b);
	}	
}
