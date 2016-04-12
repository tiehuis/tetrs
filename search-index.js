var searchIndex = {};
searchIndex["tetrs"] = {"doc":"The tetrs library provides a number of low-level tasks related to movement\nof blocks. The code aims to be correct and provide easy extension for new\ninput.","items":[[0,"field","tetrs","A generic playfield.",null,null],[3,"Field","tetrs::field","A `Field` is simply a 2-D `Vec` with some corresponding options.",null,null],[12,"width","","The width of the field.",0,null],[12,"height","","The height of the field.",0,null],[12,"hidden","","The height of the hidden region of the field.",0,null],[12,"spawn","","The initial spawn of a `Block` on this field.",0,null],[12,"data","","The current field state.",0,null],[3,"FieldOptions","","Options for use when constructing a field.",null,null],[12,"width","","",1,null],[12,"height","","",1,null],[12,"hidden","","",1,null],[12,"spawn","","",1,null],[11,"fmt","","",0,{"inputs":[{"name":"field"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"field"}],"output":{"name":"field"}}],[11,"hash","","",0,null],[11,"fmt","","",1,{"inputs":[{"name":"fieldoptions"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"deserialize","","",1,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"serialize","","",1,{"inputs":[{"name":"fieldoptions"},{"name":"__s"}],"output":{"name":"result"}}],[11,"default","","",1,{"inputs":[],"output":{"name":"fieldoptions"}}],[11,"new","","Construct a `Field` object with default values.",0,{"inputs":[],"output":{"name":"field"}}],[11,"with_options","","Construct a `Field` object with specific values.",0,{"inputs":[{"name":"fieldoptions"}],"output":{"name":"field"}}],[11,"clear_lines","","Clear lines from the field and return the number cleared.",0,{"inputs":[{"name":"field"}],"output":{"name":"usize"}}],[11,"freeze","","Freeze a block into place on the field. This takes ownership of the\nblock to ensure it cannot be used again.",0,{"inputs":[{"name":"field"},{"name":"block"}],"output":null}],[11,"get","","Return the value at the specified field location.",0,null],[11,"occupies","","Return true if the value at the specified location is non-empty.",0,null],[0,"block","tetrs","A single tetrimino.",null,null],[3,"Block","tetrs::block","A struct representing a single tetrimino.",null,null],[12,"x","","X-coordinate of the piece",2,null],[12,"y","","Y-coordinate of the piece",2,null],[12,"id","","Type of the block",2,null],[12,"r","","Rotation state of the block",2,null],[12,"rs","","Rotation system used internally",2,null],[3,"BlockOptions","","Values used for initializing a block.",null,null],[12,"x","","",3,null],[12,"y","","",3,null],[12,"rotation","","",3,null],[12,"rotation_system","","",3,null],[4,"Type","","The identifier for a particular block.",null,null],[13,"I","","",4,null],[13,"T","","",4,null],[13,"L","","",4,null],[13,"J","","",4,null],[13,"S","","",4,null],[13,"Z","","",4,null],[13,"O","","",4,null],[13,"None","","",4,null],[4,"Rotation","","Represents all rotation statuses a block can be. This is used both as\na rotation state, and to indicate how much relative movement shoud be\napplied for various functions.\nA rotation state.",null,null],[13,"R0","","",5,null],[13,"R90","","",5,null],[13,"R180","","",5,null],[13,"R270","","",5,null],[4,"Direction","","A movement along one of the four directional axes.",null,null],[13,"None","","",6,null],[13,"Left","","",6,null],[13,"Right","","",6,null],[13,"Up","","",6,null],[13,"Down","","",6,null],[11,"eq","","",4,{"inputs":[{"name":"type"},{"name":"type"}],"output":{"name":"bool"}}],[11,"fmt","","",4,{"inputs":[{"name":"type"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"type"}],"output":{"name":"type"}}],[11,"hash","","",4,null],[11,"to_usize","","",4,{"inputs":[{"name":"type"}],"output":{"name":"usize"}}],[11,"from_usize","","",4,{"inputs":[{"name":"usize"}],"output":{"name":"type"}}],[11,"variants","","Returns all `non-None` `Type` variants.",4,null],[11,"eq","","",5,{"inputs":[{"name":"rotation"},{"name":"rotation"}],"output":{"name":"bool"}}],[11,"fmt","","",5,{"inputs":[{"name":"rotation"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",5,{"inputs":[{"name":"rotation"}],"output":{"name":"rotation"}}],[11,"hash","","",5,null],[11,"to_usize","","",5,{"inputs":[{"name":"rotation"}],"output":{"name":"usize"}}],[11,"from_usize","","",5,{"inputs":[{"name":"usize"}],"output":{"name":"rotation"}}],[11,"variants","","Returns all available `Rotation` variants.",5,{"inputs":[],"output":{"name":"vec"}}],[11,"clockwise","","Returns the next clockwise rotation.",5,{"inputs":[{"name":"rotation"}],"output":{"name":"rotation"}}],[11,"anticlockwise","","Returns the next anticlockwise rotation.",5,{"inputs":[{"name":"rotation"}],"output":{"name":"rotation"}}],[11,"eq","","",6,{"inputs":[{"name":"direction"},{"name":"direction"}],"output":{"name":"bool"}}],[11,"fmt","","",6,{"inputs":[{"name":"direction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",6,{"inputs":[{"name":"direction"}],"output":{"name":"direction"}}],[11,"hash","","",6,null],[11,"variants","","Return all `non-None` `Direction` variants.",6,{"inputs":[],"output":{"name":"vec"}}],[11,"clone","","",2,{"inputs":[{"name":"block"}],"output":{"name":"block"}}],[11,"default","","",3,{"inputs":[],"output":{"name":"blockoptions"}}],[11,"new","","Construct a new default `Block` and return it.",2,{"inputs":[{"name":"type"},{"name":"field"}],"output":{"name":"block"}}],[11,"with_options","","Construct a new block with the specified options.",2,{"inputs":[{"name":"type"},{"name":"field"},{"name":"blockoptions"}],"output":{"name":"block"}}],[11,"collides","","Return `true` if the block currently collides with any pieces on the\nfield.",2,{"inputs":[{"name":"block"},{"name":"field"}],"output":{"name":"bool"}}],[11,"shift","","Shift the block one step in the specified direction.",2,{"inputs":[{"name":"block"},{"name":"field"},{"name":"direction"}],"output":{"name":"bool"}}],[11,"shift_extend","","Repeatedly shift a block as far as we can until a collision occurs.\nA HardDrop can be performed for example by calling\n`Block.shift_extend(&amp;field, Direction::Down)`.",2,{"inputs":[{"name":"block"},{"name":"field"},{"name":"direction"}],"output":null}],[11,"rotate_at_offset","","Rotate the block by a specified amount and then apply an offset.",2,null],[11,"rotate","","Rotate the block by the specified amount.",2,{"inputs":[{"name":"block"},{"name":"field"},{"name":"rotation"}],"output":{"name":"bool"}}],[11,"occupies","","Check if the block occupies a particular `(x, y)` absolute location.",2,null],[11,"ghost","","Return a `Block` which is a ghost of the current.",2,{"inputs":[{"name":"block"},{"name":"field"}],"output":{"name":"block"}}],[0,"controller","tetrs","An abstract controller for specifying actions.",null,null],[3,"Controller","tetrs::controller","A controller stores the internal state as a series of known actions.",null,null],[12,"time","","The length each action has occured for in ticks.",7,null],[12,"active","","Which actions are currently active.",7,null],[4,"Action","","Actions which are understood by the controller.",null,null],[13,"MoveLeft","","",8,null],[13,"MoveRight","","",8,null],[13,"MoveDown","","",8,null],[13,"HardDrop","","",8,null],[13,"RotateLeft","","",8,null],[13,"RotateRight","","",8,null],[13,"Hold","","",8,null],[13,"Quit","","",8,null],[11,"hash","","",8,null],[11,"fmt","","",8,{"inputs":[{"name":"action"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",8,{"inputs":[{"name":"action"}],"output":{"name":"action"}}],[11,"to_usize","","",8,{"inputs":[{"name":"action"}],"output":{"name":"usize"}}],[11,"from_usize","","",8,{"inputs":[{"name":"usize"}],"output":{"name":"action"}}],[11,"default","","",7,{"inputs":[],"output":{"name":"controller"}}],[11,"new","","Return a new controller instance.",7,{"inputs":[],"output":{"name":"controller"}}],[11,"active","","Query if an action is currently active.",7,{"inputs":[{"name":"controller"},{"name":"action"}],"output":{"name":"bool"}}],[11,"time","","Query how long an action has been active for.",7,{"inputs":[{"name":"controller"},{"name":"action"}],"output":{"name":"u64"}}],[11,"activate","","Activate the specified action.",7,{"inputs":[{"name":"controller"},{"name":"action"}],"output":null}],[11,"deactivate","","Deactivate the specified action.",7,{"inputs":[{"name":"controller"},{"name":"action"}],"output":null}],[11,"deactivate_all","","Deactivate all actions.",7,{"inputs":[{"name":"controller"}],"output":null}],[11,"update","","Update all active actions and increment their timers.",7,{"inputs":[{"name":"controller"}],"output":null}],[0,"wallkick","tetrs","Implements a wallkick test",null,null],[3,"SRS","tetrs::wallkick","Wallkick",null,null],[3,"Empty","","Wallkick",null,null],[3,"Simple","","Wallkick",null,null],[3,"DTET","","Wallkick",null,null],[5,"new","","Factory function for returning appropriate rotation systems based on their\nnames",null,{"inputs":[{"name":"str"}],"output":{"name":"wallkick"}}],[11,"new","","Return a new wallkick instance",9,{"inputs":[],"output":{"name":"srs"}}],[11,"test","","Wallkick tests for the specified id and rotation.",9,null],[11,"new","","Return a new wallkick instance",10,{"inputs":[],"output":{"name":"empty"}}],[11,"test","","",10,null],[11,"new","","Return a new wallkick instance",11,{"inputs":[],"output":{"name":"simple"}}],[11,"test","","",11,null],[11,"new","","Return a new wallkick instance",12,{"inputs":[],"output":{"name":"dtet"}}],[11,"test","","",12,null],[8,"Wallkick","","Trait which specifies what wallkick tests must implement. Every wallkick\ntest must implement an iterator with offsets of type (i32, i32).",null,null],[10,"test","","Returns a set of wallkick tests for the specified block and rotation",13,null],[0,"randomizer","tetrs","Implements a randomizer.",null,null],[3,"BagRandomizer","tetrs::randomizer","A generic bag randomizer.",null,null],[3,"MemorylessRandomizer","","A generic memoryless randomizer.\nThis generates a completely arbitrary sequence of `Type`&#39;s.",null,null],[3,"GameboyRandomizer","","A generic memoryless randomizer.\nThis generates a completely arbitrary sequence of `Type`&#39;s.",null,null],[3,"TGM1Randomizer","","A TGM1 randomizer.\nThis generates a completely arbitrary sequence of `Type`&#39;s.",null,null],[3,"TGM2Randomizer","","A TGM2 randomizer.\nThis generates a completely arbitrary sequence of `Type`&#39;s.",null,null],[5,"new","","Factory function for generating randomizers.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"bagrandomizer"}}],[11,"preview","","",14,{"inputs":[{"name":"bagrandomizer"},{"name":"usize"}],"output":{"name":"vec"}}],[11,"next","","",14,{"inputs":[{"name":"bagrandomizer"}],"output":{"name":"type"}}],[11,"clone","","",14,{"inputs":[{"name":"bagrandomizer"}],"output":{"name":"bagrandomizer"}}],[11,"new","","Generate a new `BagRandomizer` instance.",14,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[11,"preview","","",15,{"inputs":[{"name":"memorylessrandomizer"},{"name":"usize"}],"output":{"name":"vec"}}],[11,"next","","",15,{"inputs":[{"name":"memorylessrandomizer"}],"output":{"name":"type"}}],[11,"clone","","",15,{"inputs":[{"name":"memorylessrandomizer"}],"output":{"name":"memorylessrandomizer"}}],[11,"new","","Return a new `MemorylessRandomizer` instance.",15,{"inputs":[{"name":"usize"}],"output":{"name":"memorylessrandomizer"}}],[11,"preview","","",16,{"inputs":[{"name":"gameboyrandomizer"},{"name":"usize"}],"output":{"name":"vec"}}],[11,"next","","",16,{"inputs":[{"name":"gameboyrandomizer"}],"output":{"name":"type"}}],[11,"clone","","",16,{"inputs":[{"name":"gameboyrandomizer"}],"output":{"name":"gameboyrandomizer"}}],[11,"new","","Return a new `GameboyRandomizer` instance.",16,{"inputs":[{"name":"usize"}],"output":{"name":"gameboyrandomizer"}}],[11,"preview","","",17,{"inputs":[{"name":"tgm1randomizer"},{"name":"usize"}],"output":{"name":"vec"}}],[11,"next","","",17,{"inputs":[{"name":"tgm1randomizer"}],"output":{"name":"type"}}],[11,"clone","","",17,{"inputs":[{"name":"tgm1randomizer"}],"output":{"name":"tgm1randomizer"}}],[11,"new","","Return a new `TGM1Randomizer` instance.",17,{"inputs":[{"name":"usize"}],"output":{"name":"tgm1randomizer"}}],[11,"preview","","",18,{"inputs":[{"name":"tgm2randomizer"},{"name":"usize"}],"output":{"name":"vec"}}],[11,"next","","",18,{"inputs":[{"name":"tgm2randomizer"}],"output":{"name":"type"}}],[11,"clone","","",18,{"inputs":[{"name":"tgm2randomizer"}],"output":{"name":"tgm2randomizer"}}],[11,"new","","Return a new `TGM2Randomizer` instance.",18,{"inputs":[{"name":"usize"}],"output":{"name":"tgm2randomizer"}}],[8,"Randomizer","","A randomizer must implement an iterator, plus a preview function which\nreturns a number of lookahead pieces.",null,null],[10,"preview","","Return a vector containing the next `n` pieces that will be retrieved\nby the iterator.",19,{"inputs":[{"name":"randomizer"},{"name":"usize"}],"output":{"name":"vec"}}],[10,"next","","Return the next block value in this sequence.",19,{"inputs":[{"name":"randomizer"}],"output":{"name":"type"}}],[0,"rotation_system","tetrs","This modules provides an interface for dealing with different block offsets.",null,null],[5,"new","tetrs::rotation_system","Factory function for returning appropriate rotation systems based on their\nnames",null,{"inputs":[{"name":"str"}],"output":{"name":"rotationsystem"}}],[0,"srs","","Specifies the block values for the SRS rotation system.",null,null],[3,"SRS","tetrs::rotation_system::srs","",null,null],[11,"hash","","",20,null],[11,"default","","",20,{"inputs":[],"output":{"name":"srs"}}],[11,"fmt","","",20,{"inputs":[{"name":"srs"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",20,{"inputs":[{"name":"srs"}],"output":{"name":"srs"}}],[11,"new","","Return a new instance",20,{"inputs":[],"output":{"name":"srs"}}],[11,"data","","",20,null],[0,"arika","tetrs::rotation_system","Specifies the block values for the Arika rotation system.",null,null],[3,"Arika","tetrs::rotation_system::arika","",null,null],[11,"hash","","",21,null],[11,"default","","",21,{"inputs":[],"output":{"name":"arika"}}],[11,"fmt","","",21,{"inputs":[{"name":"arika"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",21,{"inputs":[{"name":"arika"}],"output":{"name":"arika"}}],[11,"new","","Return a new instance",21,{"inputs":[],"output":{"name":"arika"}}],[11,"data","","",21,null],[0,"tengen","tetrs::rotation_system","Specifies the block values for the Tengen rotation system.",null,null],[3,"Tengen","tetrs::rotation_system::tengen","",null,null],[11,"hash","","",22,null],[11,"default","","",22,{"inputs":[],"output":{"name":"tengen"}}],[11,"fmt","","",22,{"inputs":[{"name":"tengen"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",22,{"inputs":[{"name":"tengen"}],"output":{"name":"tengen"}}],[11,"new","","Return a new instance",22,{"inputs":[],"output":{"name":"tengen"}}],[11,"data","","",22,null],[0,"dtet","tetrs::rotation_system","Rotation offsrts for the DTET rotation system.",null,null],[3,"DTET","tetrs::rotation_system::dtet","",null,null],[11,"hash","","",23,null],[11,"default","","",23,{"inputs":[],"output":{"name":"dtet"}}],[11,"fmt","","",23,{"inputs":[{"name":"dtet"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",23,{"inputs":[{"name":"dtet"}],"output":{"name":"dtet"}}],[11,"new","","Return a new instance",23,{"inputs":[],"output":{"name":"dtet"}}],[11,"data","","",23,null],[8,"RotationSystem","tetrs::rotation_system","The `RotationSystem` trait is implmented by all rotation systems.",null,null],[10,"data","","Returns a static array of offset values for the specified `Type`\nand `Rotation`.",24,null],[11,"minp","","Returns the minimum offset of the first piece in a block.",24,null],[0,"engine","tetrs","Implements a high-level engine which composes all the components\ninto one abstract structure.",null,null],[3,"EngineOptions","tetrs::engine","Stores a number of internal options that may be useful during a games\nexecution.",null,null],[12,"field_options","","",25,null],[12,"randomizer_name","","",25,null],[12,"randomizer_lookahead","","",25,null],[12,"rotation_system_name","","",25,null],[12,"wallkick_name","","",25,null],[12,"mspt","","",25,null],[12,"engine_settings","","",25,null],[3,"EngineSettings","","Settings used internally by an `Engine`.",null,null],[12,"das","","DAS setting (in ms)",26,null],[12,"are","","ARE time (in ms)",26,null],[12,"gravity","","Gravity (in ms). How many ms must pass for block to fall",26,null],[12,"arr","","Auto-repeat-rate (in ms)",26,null],[12,"has_hold","","Is hold enabled",26,null],[12,"hold_limit","","How many times can we hold",26,null],[12,"has_hard_drop","","Is hard drop allowed",26,null],[12,"has_soft_drop","","Has soft drop",26,null],[12,"soft_drop_speed","","The speed soft drop works",26,null],[12,"preview_count","","Maximum number of preview pieces",26,null],[3,"Engine","","This engine allows for handling of DAS-like features and other things\nwhich are otherwise transparent to sub-components which are only\nmanaged on a per-tick basis (have no concept of state over time).",null,null],[12,"controller","","Controller which is used by the engine",27,null],[12,"randomizer","","The randomizer being used.",27,null],[12,"wallkick","","The wallkick object being used.",27,null],[12,"rotation_system","","The rotation system used by this engine.",27,null],[12,"field","","The field which the game is played on",27,null],[12,"block","","The active block",27,null],[12,"hold","","The current hold block (this doesn&#39;t store an actual block right now)",27,null],[12,"options","","Settings used internally by the engine",27,null],[12,"statistics","","Statistics of the current game",27,null],[12,"running","","Is the game running",27,null],[12,"mspt","","How many milliseconds occur per game tick.",27,null],[12,"tick_count","","How many ticks have elapsed this game",27,null],[11,"fmt","","",25,{"inputs":[{"name":"engineoptions"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"deserialize","","",25,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"serialize","","",25,{"inputs":[{"name":"engineoptions"},{"name":"__s"}],"output":{"name":"result"}}],[11,"default","","",25,{"inputs":[],"output":{"name":"engineoptions"}}],[11,"load_file","","Construct an `EngineOptions` from a file.",25,{"inputs":[{"name":"str"}],"output":{"name":"engineoptions"}}],[11,"fmt","","",26,{"inputs":[{"name":"enginesettings"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"deserialize","","",26,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"serialize","","",26,{"inputs":[{"name":"enginesettings"},{"name":"__s"}],"output":{"name":"result"}}],[11,"default","","",26,{"inputs":[],"output":{"name":"enginesettings"}}],[11,"new","","Construct a new engine object and return it.",27,{"inputs":[{"name":"engineoptions"}],"output":{"name":"engine"}}],[11,"update","","The main update phase of the engine.",27,{"inputs":[{"name":"engine"}],"output":null}],[0,"utility","tetrs","Contains a number of helper methods which are composed of a number of\nstructures.",null,null],[8,"BlockHelper","tetrs::utility","Implements new traits on a Block instance.",null,null],[10,"rotate_with_wallkick","","Rotate a block applying the specified wallkick on failures.",28,{"inputs":[{"name":"blockhelper"},{"name":"field"},{"name":"wallkick"},{"name":"rotation"}],"output":{"name":"bool"}}],[11,"rotate_with_wallkick","tetrs::block","",2,{"inputs":[{"name":"block"},{"name":"field"},{"name":"wallkick"},{"name":"rotation"}],"output":{"name":"bool"}}],[0,"statistics","tetrs","Stores statistics about the current game.",null,null],[3,"Statistics","tetrs::statistics","Stores statistics about the current game.",null,null],[12,"lines","","How many lines have been cleared",29,null],[12,"pieces","","How many pieces have been placed",29,null],[12,"singles","","Total single line clears",29,null],[12,"doubles","","Total double line clears",29,null],[12,"triples","","Total triple line clears",29,null],[12,"fours","","Total tetrises",29,null],[11,"default","","",29,{"inputs":[],"output":{"name":"statistics"}}],[11,"new","","Construct a new `Statistics` object.",29,{"inputs":[],"output":{"name":"statistics"}}],[0,"schema","tetrs","Methods for converting to and from a textual field representation.",null,null],[3,"Schema","tetrs::schema","A schema is a simple 2d textual representation of a field and a block.\nIt allows conversion from a string, and also from a `(&amp;Field, &amp;Block)` and\nbridges the gap between these two components.",null,null],[11,"fmt","","",30,{"inputs":[{"name":"schema"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",30,{"inputs":[{"name":"schema"}],"output":{"name":"schema"}}],[11,"from_state","","Construct a schema representation from an game primitives.",30,{"inputs":[{"name":"field"},{"name":"block"}],"output":{"name":"schema"}}],[11,"from_string","","Construct a schema representation from an input string.",30,{"inputs":[{"name":"str"}],"output":{"name":"schema"}}],[11,"to_state","","Constuct state objects from a given schema. This is slightly finicky\nand there are a few cases to consider.",30,null],[11,"to_string","","Construct a visual representation from a given schema.",30,{"inputs":[{"name":"schema"}],"output":{"name":"string"}}],[11,"fmt","","",30,{"inputs":[{"name":"schema"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",30,{"inputs":[{"name":"schema"},{"name":"self"}],"output":{"name":"bool"}}],[0,"import","tetrs","Convenience module providing default imports.",null,null]],"paths":[[3,"Field"],[3,"FieldOptions"],[3,"Block"],[3,"BlockOptions"],[4,"Type"],[4,"Rotation"],[4,"Direction"],[3,"Controller"],[4,"Action"],[3,"SRS"],[3,"Empty"],[3,"Simple"],[3,"DTET"],[8,"Wallkick"],[3,"BagRandomizer"],[3,"MemorylessRandomizer"],[3,"GameboyRandomizer"],[3,"TGM1Randomizer"],[3,"TGM2Randomizer"],[8,"Randomizer"],[3,"SRS"],[3,"Arika"],[3,"Tengen"],[3,"DTET"],[8,"RotationSystem"],[3,"EngineOptions"],[3,"EngineSettings"],[3,"Engine"],[8,"BlockHelper"],[3,"Statistics"],[3,"Schema"]]};
initSearch(searchIndex);