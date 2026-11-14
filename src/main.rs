//lekker is a high-level programming language(percise as interpreter)with low-level syntax,build based on rust without any external crate(before it can bootstrap itself)
/*expect syntax(current version):
	#x{12.9*31}; //# for setting the value(all will be mutable) of name between # and < inside <> and end with;
	!print{"12.9*31="x}; //function between ! and < are decide for what to do <> and abandoning the , also can do @x if need call gobal value(set outside loop)
	#y{!input{}?}; //well you can do this
	#z{["sigma",x,[y,12.3]]};
	//also deciding the and or not also () become:!and{condition1,condition2}...you got it and also the comparasion stuff,all embed to function(like !LOET{condition1,condition2} which represented larger or equal to) and the bracket become !{} also the if else also embeded to function and also loop will not count indentation but curly bracket number like !loop{};
	also somehow forgot but will use $ as setting a function/module
	uhh forgot again that module need sth for local vairble set so will use |x,y| as the normal fn sigma(x,y) (also used in if else like :
	
		!if|!equal{@x,@y}?|{!loop{blablabla...}?};
		!else{!loop{blablabla..}?};
		
	also if nothing inside the || still need put in $simga||{};
	ahhhh gaining headache on this fahhhhhhhhhhhhhhhhhhhhhhh
	i have the fucking chinese test need to study and still being jobless doing this ass holly
	if i really success to make it work somehow i guess i am going to...uhh download ig and show it to everyone i guess(kinda hate using ig)
	for the shit if have loop in loop then i will probably feed it to itself..so variable will reset become local in every !{}? bracket bruh omg so much shit on this ass project bruh
	uhh kinda annoying
*/
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
#[derive(Debug,Clone)]
enum types{
	end,
	innerend,
	function,
	maths,
	number,
	string,
	boolean,//^ || ~ to represent
	slist,
	elist,
	scurly,
	ecurly,
	svarb,
	evarb,//the |x,y| during $sigma|x,y|{
	variable,
	separater,//the , which cant use in print
	breaker,//( to represent
	continuer,//) to represent
	attribute,// the || which used for the |x,y| also maybe for if else condition checking
}
enum variable_type{
	number(f64),
	string(String),
	boolean(bool),
	list(Vec<variable_type>),
}
enum math{
	add,
	minus,
	divide,
	times,
}
//the enum math and function might abandon or idk embed into the execution part i guess
enum functions{
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
	funcnamed(String),//for module like $sigma||{};
	get,//for access the variables inside a list !get|x|{[index,index]};
	import,//for future to make import library bruh
}
enum func{
	callfunction,//perform the !
	pullfromglobal,//perform the @
	assign,//perform the #
	fnset,//perform the $
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
				else if "+-*/".contains(&i){chartype.push(types::maths);chartemp.push(i.to_string());}
				else if "^~".contains(&i){chartype.push(types::boolean);chartemp.push(i.to_string());}
				else if "!#$".contains(&i){chartype.push(types::function);chartemp.push(i.to_string());}//put @to variable instead of a function
				else if i==";"{chartype.push(types::end);chartemp.push(i.to_string());}
				else if i=="?"{chartype.push(types::innerend);chartemp.push(i.to_string());}
				else if i=="["{chartype.push(types::slist);chartemp.push(i.to_string());}
				else if i=="]"{chartype.push(types::elist);chartemp.push(i.to_string());}
				else if i=="{"{chartype.push(types::scurly);chartemp.push(i.to_string());}
				else if i=="}"{chartype.push(types::ecurly);chartemp.push(i.to_string());}
				else if i=="("{chartype.push(types::breaker);chartemp.push(i.to_string());}
				else if i==")"{chartype.push(types::continuer);chartemp.push(i.to_string());}
				else if i==","{chartype.push(types::separater);chartemp.push(i.to_string());}
				else if i=="|"{chartype.push(types::attribute);chartemp.push(i.to_string());}
				else{chartype.push(types::variable);chartemp.push(i.to_string());}
			}
		}
	}
	(chartype,chartemp)
}
//recieve the result from lexer and start check the grammar also merge the char and types into one Vec<variable_type>
//remember to create a whole new enum for this like enum store everything so it can execute it i guess
//first need merge and flagging error btw i am kinda wondering how to handle the uhh loop?
//ah probably handling just do in execution then,here just do merge and error flagging
//bundle the codeblock into Vec<everything> like if loop then uhh a larger inside Vec<everything i guess bruh
//here only do the first step of merging,separating by codeblocks (;) as separation
//urr today done this is kinda enough,my brain tired bruh even though just wake up for 2 hour already tired of this ass
fn parser_init(bruh:(Vec<types>,Vec<String>))->(Vec<Vec<types>>,Vec<Vec<String>>){
	let (chartype,chartemp)=bruh;
	let mut blocktemp:(Vec<Vec<types>>,Vec<Vec<String>>)=(Vec::new(),Vec::new());
	let mut temp:Vec<types>=Vec::new();
	let mut ctemp:Vec<String>=Vec::new();
	for i in 0..chartype.len(){
		match chartype[i]{
			types::end=>{blocktemp.0.push(temp);blocktemp.1.push(ctemp.clone());temp=Vec::new()},
			_=>{temp.push(chartype[i].clone());ctemp.push(chartemp[i].clone());},
		}
	}//i think done the init separation? later will check grammar inside each temp adn ctemp i guess
	//just print it out to sure it first what left is see inside each block to translate all into enum btw make a new one for this then which function identification just do it in execution then,just check grammar is fine like check bracket num sth like that i guess
	blocktemp
}
//here will try merge each code block or flag error
//damn maybe stop coding 2 hour later,i need study chinese test bruh
fn parser_second(damn:(Vec<Vec<types>>,Vec<Vec<String>>)){
	let (cdtype,cdtemp)=damn;
	for i in 0..cdtype.len(){
		for j in 0..cdtype[i].len(){
			//errrrrr here try merge then,write tmr today kinda tired from this ass also write a enum instead of a vec to store in this step
		}
	}
}
struct lekker{
	codebyblock:String,//uhh change it later then
	global_variable:HashMap<String,variable_type>,
	temp:Vec<HashMap<String,variable_type>>,
	local_variable:HashMap<String,variable_type>,
	current_block:usize,
	named_function:HashMap<String,usize>//name of that function and line to jump to
}
//rememebr to build a "thing" to check if the file ends with .lek uhh and also change the github .lekker file into .lek
fn main(){
	let path:Vec<_>=env::args().collect();
	if path.len()!=2{
		eprint!("Error:Arguments too much or not enough,suppose should be ./lekker [your file] || lekker.exe [your file]");
		exit(1)
	}else{
		let content:String=fs::read_to_string(path[1].clone()).unwrap().to_string();
		let (a,b)=lexer(content);
		println!("code lexed:{:?}\n\nraw:{:?}",a,b);
		let  (c,d)=parser_init((a,b));
		println!("\nfirst init prarsed:{:?}\n\nraw init parsed:{:?}",c,d);
	}	
}
