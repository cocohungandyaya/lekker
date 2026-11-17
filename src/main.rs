/*//lekker is a high-level programming language(percise as interpreter)with low-level syntax,build based on rust without any external crate(before it can bootstrap itself)
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
#[derive(Debug,Clone,PartialEq)]
enum types{
	end,
	innerend,
	function,
	number,
	string,
	boolean,//^ || ~ to represent
	slist,
	elist,
	scurly,
	ecurly,
	variable,
	separater,//the , which can use in print
	attribute,// the || which used for the |x,y| also maybe for if else condition checking
}//POV:you wake up at 12:22 and first thing you do is coding bruh,i need get a job holly
enum action{
	assign{varname:String,value:Box<action>},
	call{varname:String,global:bool},
	function{fnname:functions,attributes:Vec<action>,inneraction:Vec<action>},
	modset{modname:functions,attributes:Vec<action>,inneraction:Vec<action>},//urrr i think i kinda suck bruh forgot about my chinese test again fahhhh
	number(f64),
	string(String),
	boolean(bool),
	list(Vec<variable_type>),
	breaker,
	continuer,
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
	get,//for access the variables inside a list !get|[index,index]|{x};
	change,//for change the variables indide a list !change|[index,index],change|{x}
	append,//for add variables in a list !append|[index,index],variable|{x}
	remove,//for remove variables
	import,//for future to make import library bruh
	len,
	add,
	minus,
	divide,
	times,
	breaker,
	continuer,
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
				else if "^~".contains(&i){chartype.push(types::boolean);chartemp.push(i.to_string());}
				else if "!#$".contains(&i){chartype.push(types::function);chartemp.push(i.to_string());}//put @to variable instead of a function
				else if i==";"{chartype.push(types::end);chartemp.push(i.to_string());}
				else if i=="?"{chartype.push(types::innerend);chartemp.push(i.to_string());}
				else if i=="["{chartype.push(types::slist);chartemp.push(i.to_string());}
				else if i=="]"{chartype.push(types::elist);chartemp.push(i.to_string());}
				else if i=="{"{chartype.push(types::scurly);chartemp.push(i.to_string());}
				else if i=="}"{chartype.push(types::ecurly);chartemp.push(i.to_string());}
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
			types::end=>{blocktemp.0.push(temp);blocktemp.1.push(ctemp.clone());temp=Vec::new();ctemp=Vec::new();},
			_=>{temp.push(chartype[i].clone());ctemp.push(chartemp[i].clone());},
		}
	}//i think done the init separation? later will check grammar inside each temp adn ctemp i guess
	//just print it out to sure it first what left is see inside each block to translate all into enum btw make a new one for this then which function identification just do it in execution then,just check grammar is fine like check bracket num sth like that i guess
	blocktemp
}
//here will try merge each code block or flag error
//damn maybe stop coding 2 hour later,i need study chinese test bruh
fn parser_second(damn:(Vec<Vec<types>>,Vec<Vec<String>>))->(Vec<Vec<types>>,Vec<Vec<String>>){
	let (cdtype,cdtemp)=damn;
	let mut temp_tlist:Vec<types>=Vec::new();
	let mut temp_clist:Vec<String>=Vec::new();
	let mut final_merged:(Vec<Vec<types>>,Vec<Vec<String>>)=(Vec::new(),Vec::new());
	for i in 0..cdtype.len(){
		for j in 0..cdtype[i].len(){
//fahhh now 13:00 bruh ytd(actually today) sleep at 6am bruh holly kinda tired bruh fahhhhhhhhhhhhhhhhh
//first might be merging each char into meaningful small blocks in a code block
//ロキシー・ミグルディアは最高!!!!!gugugaga...might just stop coding at 2 pm bruh i need study chinese bruh
//errr what to write actually...Songs that write by Tana.H is absolute cinema bruh also 9lana voice just superb bruh
//errr joblessssss,best unemployed moment huh...i dont want to go to school bruh school suck...
//fahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
//i think might be add things to the list and check if same type so then merge them into same cell?? who knows bruh
//"iris out" is a good song pal...just maybe a little bit too loud,suggest lower the volume to listen(i mean the original not other cover)
//now 13:34!? err half hour writing nothing i kinda suck bruh
//oh yeah so lekker code will be all start with like !#$ that is it so maybe that easier to code!? like get first and then till {  i guess?
//yeah seems the facts is now is 13:40 and i know how bruh cuz all start with !#$ so just get the first and check if it is uhh...function?
//song:シャーデンフロイデ / ぐちり feat. 音街ウナ absolute cinema must watch pal...
//golden brown also absolute cinema actually my fav bruh
//kinda tired i dont want coding...bruh you know what i do? spend 48 minutes writing comments and actually write onlt 5 line of codes...err get a job...
			if j>0{//holly where is the bug oh jeez...bruh i found it, not here,here is perfect the problem is parser_init ctemp forgot to reset bruh
				if cdtype[i][j]== temp_tlist[temp_tlist.len()-1]{
//here do merging yeah maybe parser_second only do merging then not even error flagging bruh holly i suck maybe parser_third do the error flagging i guess?
//omg i hate errors so fucking annoying
					let id=temp_clist.len();
					temp_clist[id-1]=format!("{}{}",temp_clist[id-1],cdtemp[i][j].clone());
				}//that is it!?
				else{temp_tlist.push(cdtype[i][j].clone());temp_clist.push(cdtemp[i][j].clone());}
			}else{temp_tlist.push(cdtype[i][j].clone());temp_clist.push(cdtemp[i][j].clone());}
			//err and rese the two temp_tlist i guess and make a final
		}
		final_merged.0.push(temp_tlist);final_merged.1.push(temp_clist);
		temp_tlist=Vec::new();
		temp_clist=Vec::new();
	}
	final_merged
}//finally done this... somehow i suppose can done in 30 min but i used 1 hour to write tons of useless comments bruh holly i really suck bruh
fn parser_third(fahhh:(Vec<Vec<types>>,Vec<Vec<String>>),code_block:Vec<usize>)->Vec<action>{
	//here is probably for uhh flagging error and final merge
	let (fah1,fah2)=fahhh;
	let mut codeblock=code_block;
	let mut ttemp:Vec<types>=Vec::new();
	let mut ctemp:Vec<String>=Vec::new();
	let mut ittemp:Vec<types>=Vec::new();
	let mut ictemp:Vec<String>=Vec::new();
	let mut lpnum:i32=0;
	let mut been_loop:bool=false;
	let mut final:Vec<action>=Vec::new();
	for i in 0..fah1.len(){
		been_loop=true;
		for j in 0..fah1[i].len(){
			match fah1[i][j].clone(){
				types::scurly=>{lpnum+=1;if lpnum==1{continue;}},
				types::ecurly=>{lpnum-=1;if lpnum==1{continue;}},
			}
			if lpnum<0{eprint!(codeblock.push(i);"Syntax Error:Missing Opening Bracket...Code_block:{:?}",codeblock);exit(1);}
			else if lpnum >0 &&j==fah1[i].len()-1{codeblock.push(i);eprint!("Syntax Error:Missing Closing Bracket...Code_block:{:?}",codeblock);exit(1);}
			else{
				if lpnum==0{ttemp.push(fah1[i][j].clone());ctemp.push(fah2[i][j].clone());}
				else{ittemp.push(fah1[i][j].clone());ictemp.push(fah2[i][j].clone());}
			}
		}
//err my brain now got filled with 雨愛 (Rainie Love) i just dont get it,like it is a good song,why put it with 咒術迴戰->新宿決戰!? just ruined the song bruh holly
//i am deciding add back , to uhh print bruh kinda suck might actually can just make a list for that bruh
//to lower the risk of dumb people why dont make everything into functions like must use !{} sth like that? like even add minus all throw into function huh that way easier for me to check eror isnt it huh so i just change almost everything into a function except still the # and @ and $ bruh
		//stuck here for an hour finally kinda understand what to do bruh
		if ttemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Missing sth before bracket example:!print{};...Code_block:{}",codeblock);exit(1);}
		else{
			let mut fahtemp:Vec<types>=Vec::new();
			let mut fahctemp:Vec<String>=Vec::new();
			let mut inat:bool=false;
			let mut attemp:Vec<types>=Vec::new();
			let mut actemp:Vec<String>=Vec::new();//err i need rest bruh
			let mut temp:Vec<action>=Vec::new();
			let mut atemp:Vec<action>=Vec::new();
			for x in 0..ttemp.len(){
				if inat{
					match ttemp[x].clone(){
						type::attribute=>{inat =!inat;},
						type::separater=>{},
						_=>{attemp.push(ttemp[x].clone());actemp.push(ctemp[x].clone());},
					}
				}else{//arrr 1 hour wasted again god damn
					match ttemp[x].clone(){
						type::attribute=>{inat=!inat;},
						type::variable=>{fahtemp.push(ttemp[x].clone());fahctemp.push(ctemp[x].clone());},
						_=>{codeblock.push(i);eprint!("Type Error:Supposingly you should use type::variable for name/call/assign...Code_block:{}",codeblock);exit(1);},
					}
					
				}
			}
			if fahtemp.len()>2{codeblock.push(i);eprint!("Syntax Error:Supposingly you should give only 2 args(not attribute) before a bracket...Code_block:{}",codeblock);exit(1);}
			else if fahtemp.len()<2{
				if "!"==fahctemp[0]{
					if attemp.is_empty(){
						if !ittemp.is_empty(){
							//fahhh 1 hour pass again bruh
							//i should split the ? but with counting the inloop num bruh
							let mut inloop:i32=0;
							let mut bruhtemp:Vec<types>=Vec::new();
							let mut bruhctemp:Vec<String>=Vec::new();
							let mut absolutetemp:Vec<types>=Vec::new();
							let mut absolutectemp:Vec<String>=Vec::new();
							//fahhh building up spaghetti code again jeez...
							//初音ミク!!! yeah i am currently listening to a song call Miku(song by Anamanaguchi) yeah that is it,meaningless comments again bruh
							for huh in 0..ittemp.len(){
								//whatever even need rewrite just rewrite this part is okay bruh
								match ittemp[huh].clone(){
									types::innerend=>{
										if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
										//ahh my brain is melting bruh
										else if inloop==0{
											bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
											bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
										}else{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());}
										},
									types::scurly=>{inloop+=1;continue;},
									types::ecurly=>{inloop-=1;continue;},
									_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
								}
							}
							//i not even sure is it ok bruh brain kinda cooked
							codeblock.push(i);	final.push(action::function{fnname:functions::bracket,attributes:Vec::new(),inneraction:parser_third((bruhtemp,bruhctemp),codeblock)});
							codeblock.pop().unwrap();
						}
						else{//function{fnname:functions,attributes:Vec<action>,inneraction:Vec<action>}
							final.push(action::function{fnname:functions::bracket,attributes:Vec::new(),inneraction:Vec::new()});
						}
					}else{
						if !ittemp.is_empty(){
							let mut inloop:i32=0;
							let mut bruhtemp:Vec<types>=Vec::new();
							let mut bruhctemp:Vec<String>=Vec::new();
							let mut absolutetemp:Vec<types>=Vec::new();
							let mut absolutectemp:Vec<String>=Vec::new();
							for huh in 0..ittemp.len(){
								match ittemp[huh].clone(){
									types::innerend=>{
										if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
										else if inloop==0{
											bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
											bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
										}else{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());}
										},
									types::scurly=>{inloop+=1;continue;},
									types::ecurly=>{inloop-=1;continue;},
									_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
								}
							}
							codeblock.push(i);
							temp=parser_third((bruhtemp,bruhctemp),codeblock);
							codeblock.pop().unwrap();
							let mut inloop:i32=0;
							let mut bruhtemp:Vec<types>=Vec::new();
							let mut bruhctemp:Vec<String>=Vec::new();
							let mut absolutetemp:Vec<types>=Vec::new();
							let mut absolutectemp:Vec<String>=Vec::new();
							for huh in 0..attemp.len(){
								match attemp[huh].clone(){
									types::innerend=>{
										if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
										else if inloop==0{
											bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
											bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
										}else{absolutetemp.push(attemp[huh].clone());absolutectemp.push(actemp[huh].clone());}
										},
									types::scurly=>{inloop+=1;continue;},
									types::ecurly=>{inloop-=1;continue;},
									types::separater=>{},
									_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
								}
							}
							codeblock.push(i);
							atemp=parser_third((bruhtemp,bruhctemp),codeblock);
							codeblock.pop().unwrap();
								final.push(action::function{fnname:functions::bracket,attributes:atemp.clone(),inneraction:temp.clone()});
						}
						else{
							temp=parser_third((bruhtemp,bruhctemp),codeblock);
							let mut inloop:i32=0;
							let mut bruhtemp:Vec<types>=Vec::new();
							let mut bruhctemp:Vec<String>=Vec::new();
							let mut absolutetemp:Vec<types>=Vec::new();
							let mut absolutectemp:Vec<String>=Vec::new();
							for huh in 0..attemp.len(){
								match attemp[huh].clone(){
									types::innerend=>{
										if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
										else if inloop==0{
											bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
											bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
										}else{absolutetemp.push(attemp[huh].clone());absolutectemp.push(actemp[huh].clone());}
										},
									types::scurly=>{inloop+=1;continue;},
									types::ecurly=>{inloop-=1;continue;},
									types::separater=>{},
									_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
								}
							}
							codeblock.push(i)
							atemp=parser_third((bruhtemp,bruhctemp),codeblock);
							codeblock.pop().unwrap();
							final.push(action::function{fnname:functions::bracket,attributes:atemp.clone(),inneraction:Vec::new()});
						}
					}
				}
				else{codeblock.push(i);eprint!("Syntax Error:Supposingly only ! can put nothing between it and the bracket...Code_block:{}",codeblock);exit(1);}
			}
			else{
				if "!"==fahctemp[0]{
					match fahtemp[1]{
						types::variable{
							let mut fahbruh:function;
							if fahctemp[1]=="print"{fahbruh=function::print;}
							else if fahtemp[1]=="input"{fahbruh=function::input;}
							else if fahtemp[1]=="equal"{fahbruh=function::equal;}
							else if fahtemp[1]=="lt"{fahbruh=function::lt;}
							else if fahtemp[1]=="st"{fahbruh=function::st;}
							else if fahtemp[1]=="not"{fahbruh=function::not;}
							else if fahtemp[1]=="and"{fahbruh=function::and;}
							else if fahtemp[1]=="or"{fahbruh=function::or;}
							else if fahtemp[1]=="loop"{fahbruh=function::lloop;}
							else if fahtemp[1]=="if"{fahbruh=function::lif;}
							else if fahtemp[1]=="else"{fahbruh=function::lelse;}
							else if fahtemp[1]=="intolist"{fahbruh=function::intolist;}
							else if fahtemp[1]=="st"{fahbruh=function::st;}
							else if fahtemp[1]=="fsread"{fahbruh=function::fsread;}
							else if fahtemp[1]=="fswrite"{fahbruh=function::fswrite;}
							else if fahtemp[1]=="get"{fahbruh=function::get;}
							else if fahtemp[1]=="change"{fahbruh=function::change;}
							else if fahtemp[1]=="append"{fahbruh=function::append;}
							else if fahtemp[1]=="remove"{fahbruh=function::remove;}
							else if fahtemp[1]=="import"{fahbruh=function::import;}
							else if fahtemp[1]=="len"{fahbruh=function::len;}
							else if fahtemp[1]=="add"{fahbruh=function::add;}
							else if fahtemp[1]=="minus"{fahbruh=function::minus;}
							else if fahtemp[1]=="divide"{fahbruh=function::divide;}
							else if fahtemp[1]=="times"{fahbruh=function::times;}
							else if fahtemp[1]=="break"{fahbruh=function::breaker;}
							else if fahtemp[1]=="continue"{fahbruh=function::continuer;}
							else{fahbruh=function::funcnamed(fahtemp[1].clone())}
							if attemp.is_empty(){
								if !ittemp.is_empty(){
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..ittemp.len(){
										match ittemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									codeblock.push(i);
										final.push(action::function{fnname:fahbruh,attributes:Vec::new(),inneraction:parser_third((bruhtemp,bruhctemp),codeblock)});
									codeblock.pop().unwrap();
								}
								else{
									codeblock.push(i);
									final.push(action::function{fnname:fahbruh,attributes:Vec::new(),inneraction:Vec::new()});
									codeblock.pop().unwrap();
								}
							}else{
								if !ittemp.is_empty(){
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..ittemp.len(){
										match ittemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									temp=parser_third((bruhtemp,bruhctemp),codeblock);
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..attemp.len(){
										match attemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(attemp[huh].clone());absolutectemp.push(actemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											types::separater=>{},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									codeblock.push(i);
									atemp=parser_third((bruhtemp,bruhctemp),codeblock);
									codeblock.pop().unwrap();
									final.push(action::function{fnname:fahbruh,attributes:atemp.clone(),inneraction:temp.clone()});
								}
								else{
									temp=parser_third((bruhtemp,bruhctemp),codeblock);
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..attemp.len(){
										match attemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(attemp[huh].clone());absolutectemp.push(actemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											types::separater=>{},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									codeblock.push(i);
									atemp=parser_third((bruhtemp,bruhctemp),codeblock);
									codeblock.pop().unwrap();
									final.push(action::function{fnname:fahbruh,attributes:atemp.clone(),inneraction:Vec::new()});
								}
							}
						},
						_=>{codeblock.push(i);eprint!("Syntax Error:Supposingly you should use variable type to call functions...Code_block:{}",codeblock);exit(1);}
					}
				}//ain no lie brain melting so fucking tired bruh this is horrible
				//assign{varname:String,value:Box<action>}
				else if "#"==fahctemp[0]{
					match fahtemp[1]{
						types::variable{
							if attemp.is_empty(){
								if !ittemp.is_empty(){
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..ittemp.len(){
										match ittemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									codeblock.push(i);
									let blablabla=parser_third((bruhtemp,bruhctemp),codeblock);
									codeblock.pop().unwrap();
									if blablala.len()>1{
										codeblock.push(i);eprint!("Syntax Error:Supposingly you shouldnt put multiple stuff inside a assign function...Code_block:{}",codeblock);exit(1);}else{
										final.push(action::assign{varname:fahctemp[1],value:Box::new(blablabla[0])});
									}
								}
								else{
									codeblock.push(i);eprint!("Syntax Error:Supposingly you should at least put sth inside the assign function...Code_block:{}",codeblock);exit(1);
								}
							}else{
								codeblock.push(i);eprint!("Syntax Error:Supposingly there shouldnt be any attribute when assigning variables...Code_block:{}",codeblock);exit(1);
							}
						},
						_=>{codeblock.push(i);eprint!("Syntax Error:Supposingly you should use variable type to assign functions...Code_block:{}",codeblock);exit(1);}
					}
				}
				else if "$"==fahctemp[0]{
					match fahtemp[1]{
						types::variable{
							let mut fahbruh:function;
							if fahctemp[1]=="print"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="input"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="equal"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="lt"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="st"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="not"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="and"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="or"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="loop"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="if"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="else"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="intolist"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="st"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="fsread"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="fswrite"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="get"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="change"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="append"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="remove"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="import"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="len"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="add"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="minus"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="divide"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="times"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="break"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else if fahtemp[1]=="continue"{codeblock.push(i);eprint!("Syntax Error:you cant make module by the name of a preset function...Code_block:{}",codeblock);exit(1);}
							else{fahbruh=function::funcnamed(fahtemp[1].clone())}
							if attemp.is_empty(){
								if !ittemp.is_empty(){
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..ittemp.len(){
										match ittemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									codeblock.push(i);
										final.push(action::modset{modname:fahbruh,attributes:Vec::new(),inneraction:parser_third((bruhtemp,bruhctemp),codeblock)});
									codeblock.pop().unwrap();
								}
								else{
									final.push(action::modset{modname:fahbruh,attributes:Vec::new(),inneraction:Vec::new()});
								}
							}else{
								if !ittemp.is_empty(){
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..ittemp.len(){
										match ittemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									codeblock.push(i);
									temp=parser_third((bruhtemp,bruhctemp),codeblock);
									codeblock.pop().unwrap();
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..attemp.len(){
										match attemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(attemp[huh].clone());absolutectemp.push(actemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											types::separater=>{},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									codeblock.push(i);
									atemp=parser_third((bruhtemp,bruhctemp),codeblock);
									codeblock.pop().unwrap();
									final.push(action::modset{modname:fahbruh,attributes:atemp.clone(),inneraction:temp.clone()});
								}
								else{
									temp=parser_third((bruhtemp,bruhctemp),codeblock);
									let mut inloop:i32=0;
									let mut bruhtemp:Vec<types>=Vec::new();
									let mut bruhctemp:Vec<String>=Vec::new();
									let mut absolutetemp:Vec<types>=Vec::new();
									let mut absolutectemp:Vec<String>=Vec::new();
									for huh in 0..attemp.len(){
										match attemp[huh].clone(){
											types::innerend=>{
												if absolutetemp.is_empty(){codeblock.push(i);eprint!("Syntax Error:Supposingly you at least put something before ?...Code_block:{}",codeblock);exit(1);}
												else if inloop==0{
													bruhtemp.push(absolutetemp.clone();absolutetemp=Vec::new(););
													bruhctemp.push(absolutectemp.clone();absolutectemp=Vec::new(););
												}else{absolutetemp.push(attemp[huh].clone());absolutectemp.push(actemp[huh].clone());}
												},
											types::scurly=>{inloop+=1;continue;},
											types::ecurly=>{inloop-=1;continue;},
											types::separater=>{},
											_=>{absolutetemp.push(ittemp[huh].clone());absolutectemp.push(ictemp[huh].clone());},
										}
									}
									codeblock.push(i);
									atemp=parser_third((bruhtemp,bruhctemp),codeblock);
									codeblock.pop().unwrap();
									final.push(action::modset{modname:fahbruh,attributes:atemp.clone(),inneraction:Vec::new()});
								}
							}
						},
						_=>{codeblock.push(i);eprint!("Syntax Error:Supposingly you should use variable type to call functions...Code_block:{}",codeblock);exit(1);}
					}
				}//modset{modname:String,attributes:Vec<action>,inneraction:Vec<action>}
				//omg wtf am i even doing,time to rest bruh so tired,today end here bruh
			}
						
struct lekker{
	codebyblock:Vec<action>,//uhh change it later then
	funclist:HashMap<Vec<String>,Vec<action>>,//as final i decide still us String
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
		let  (e,f)=parser_second((c,d));
		println!("\nfirst second prarsed:{:?}\n\nraw second parsed:{:?}",e,f);
	}	
}
*/
//upper is old code,gonna rewrite everything here,i have think for this quite a long time...i kinda suck so rewrite it then
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
	yeah i finally used my brain,if now rules is make by me why dont make it whatever i want lmao
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
	bl(bool),
	list(Vec<variables>),
	Null,
}
struct lekker{
	global_variables:HashMap<String,variables>,
	error_log:Vec<String>,
	local_variables:HashMap<String,variables>,
	local_temp:Vec<HashMap<String,variables>,
	code:String,
	linenum:usize,
	error_flag:bool,
}
impl lekker{
	fn sbyblock(mut &self)->(variabes,usize){
		//make a loop for action oneach line,syntax must all separated by rather \t or " " or else error(skip that line)
		let mut temp:String=String::new();
		let mut list_temp:Vec<String>=Vec::new();
		let mut final:Vec<Vec<String>>=Vec::new()://Only Way sing by 9lana is actually cool
		let mut fahstring:bool=true;//also means in_string and true is meaning it isnt in the string
		let mut lpnum:i32=0;
		let sigma=self.code.clone().split("").collect::<Vec<_>>();
		let mut iscomment:bool=true;//reverse actually
		for i in 0..sigma.len(){
			if lpnum==0{
				if fahstring{
					if iscomment{
						if "\""==code[i].clone(){
							fahstring=!fahstring;
						}else if " "==code[i].clone() || "\t"==code[i].clone(){
							if !temp.is_empty(){
								list_temp.push(temp.clone());
								temp=String::new();
							}
						}else if "\n"==code[i].clone(){
							if !list_temp.is_empty(){
								final.push(list_temp.clone());
							}
							else{final.push(Vec::new());}
							list_temp=Vec::new();
							temp=String::new();
						}else if "{"==code[i].clone(){
							lpnum+=1;//anyone can help me with my chemistry...i really need help on that...
							temp.push_str(code[i].as_str());
						}
						else if "}"==code[i].clone(){
							if !list_temp.is_empty(){
								final.push(list_temp.clone());
							}
							list_temp=Vec::new();
							temp=String::new();
							break;
						}else if "#"==code[i].clone(){
							iscomment= !iscomment;
						}else{
							temp.push_str(code[i].as_str());
						}
					}else{
						if "#" == code[i].clone(){
							iscomment= !iscomment;
						}else if "\n"==code[i].clone(){
							if !list_temp.is_empty(){
								final.push(list_temp.clone());
							}
							else{final.push(Vec::new());}
							list_temp=Vec::new();
							temp=String::new();
							iscomment= !iscomment;
						}
					}
				}else{
					if "\""==code[i].clone(){
						fahstring=!fahstring;
					}else if "\n"==code[i].clone() || i==sigma.len()-1{
						if !list_temp.is_empty(){
							temp.push_str("\"");
							list_temp.push(temp.clone());
							final.push(list_temp.clone());
						}else{
							final.push(Vec::new());
						}
						list_temp=Vec::new();
						temp=String::new();
					}
					else{
						temp.push_str(code[i].as_str());
					}
				}
			}else{
				if fahstring{//holly this week suck tmr english test and the day after is fucking chinese test
					//9lana absolute cinema!(imagine here have a sticker bruh)
					if "\""==code[i].clone(){
						fahstring=!fahstring;
					}else if "{"==code[i].clone(){
						lpnum+=1;
						temp.push_str(code[i].as_str());
					}else if "}"==code[i].clone(){
						lpnum-=1;
						temp.push_str(code[i].as_str());
					}else{
						temp.push_str(code[i].as_str());
					}
				}else{//いますぐ輪廻!
					//golden brown also actually absolute cinema cuz that is my favourite
					if "\""==code[i].clone(){
						fahstring=!fahstring;
					}else if i==sigma.len()-1{
						if !list_temp.is_empty(){
							temp.push_str("\"");
							list_temp.push(temp.clone());
							final.push(list_temp.clone());
						}else{
							final.push(Vec::new());
						}
						list_temp=Vec::new();
						temp=String::new();
					}else{
						temp.push_str(code[i].as_str());
					}//actually just doing copy and paste bruh
				}
			}
		}//tmr start from here,today job done,tmr start write the execution part bruh
	}
}//actually i can wrap it inside itself when encounter loop bruh

	
fn main(){
	let path:Vec<_>=env::args().collect();
	if path.len()!=2{
		eprint!("Error:Arguments too much or not enough,suppose should be ./lekker [your file] || lekker.exe [your file]");
		exit(1)
	}else{
		let content:String=fs::read_to_string(path[1].clone()).unwrap().to_string();
		
	}	
}
