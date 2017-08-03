var searchIndex = {};
searchIndex["log"] = {"doc":"A lightweight logging facade.","items":[[3,"LogRecord","log","The \"payload\" of a log message. This structure is primarily used as a parameter in the [`log`] method of the [`Log`] trait.",null,null],[3,"LogMetadata","","Metadata about a log message.",null,null],[3,"LogLocation","","The location of a log message.",null,null],[3,"MaxLogLevelFilter","","A token providing read and write access to the global maximum log level filter.",null,null],[3,"SetLoggerError","","The type returned by `set_logger` if `set_logger` has already been called.",null,null],[3,"ShutdownLoggerError","","The type returned by `shutdown_logger_raw` if `shutdown_logger_raw` has already been called or if `set_logger_raw` has not been called yet.",null,null],[4,"LogLevel","","An enum representing the available verbosity levels of the logging framework.",null,null],[13,"Error","","The \"error\" level.",0,null],[13,"Warn","","The \"warn\" level.",0,null],[13,"Info","","The \"info\" level.",0,null],[13,"Debug","","The \"debug\" level.",0,null],[13,"Trace","","The \"trace\" level.",0,null],[4,"LogLevelFilter","","An enum representing the available verbosity level filters of the logging framework.",null,null],[13,"Off","","A level lower than all log levels.",1,null],[13,"Error","","Corresponds to the `Error` log level.",1,null],[13,"Warn","","Corresponds to the `Warn` log level.",1,null],[13,"Info","","Corresponds to the `Info` log level.",1,null],[13,"Debug","","Corresponds to the `Debug` log level.",1,null],[13,"Trace","","Corresponds to the `Trace` log level.",1,null],[5,"max_log_level","","Returns the current maximum log level.",null,{"inputs":[],"output":{"name":"loglevelfilter"}}],[5,"set_logger","","Sets the global logger.",null,{"inputs":[{"name":"m"}],"output":{"name":"result"}}],[5,"set_logger_raw","","Sets the global logger from a raw pointer.",null,{"inputs":[{"name":"m"}],"output":{"name":"result"}}],[5,"shutdown_logger","","Shuts down the global logger.",null,{"inputs":[],"output":{"name":"result"}}],[5,"shutdown_logger_raw","","Shuts down the global logger.",null,{"inputs":[],"output":{"name":"result"}}],[8,"Log","","A trait encapsulating the operations required of a logger",null,null],[10,"enabled","","Determines if a log message with the specified metadata would be logged.",2,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"bool"}}],[10,"log","","Logs the `LogRecord`.",2,{"inputs":[{"name":"self"},{"name":"logrecord"}],"output":null}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",0,null],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"loglevel"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"loglevel"}],"output":{"name":"bool"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"loglevelfilter"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",0,{"inputs":[{"name":"self"},{"name":"loglevel"}],"output":{"name":"option"}}],[11,"partial_cmp","","",0,{"inputs":[{"name":"self"},{"name":"loglevelfilter"}],"output":{"name":"option"}}],[11,"cmp","","",0,{"inputs":[{"name":"self"},{"name":"loglevel"}],"output":{"name":"ordering"}}],[11,"from_str","","",0,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"max","","Returns the most verbose logging level.",0,{"inputs":[],"output":{"name":"loglevel"}}],[11,"to_log_level_filter","","Converts the `LogLevel` to the equivalent `LogLevelFilter`.",0,{"inputs":[{"name":"self"}],"output":{"name":"loglevelfilter"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",1,null],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"loglevelfilter"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"loglevelfilter"}],"output":{"name":"bool"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"loglevel"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",1,{"inputs":[{"name":"self"},{"name":"loglevelfilter"}],"output":{"name":"option"}}],[11,"partial_cmp","","",1,{"inputs":[{"name":"self"},{"name":"loglevel"}],"output":{"name":"option"}}],[11,"cmp","","",1,{"inputs":[{"name":"self"},{"name":"loglevelfilter"}],"output":{"name":"ordering"}}],[11,"from_str","","",1,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"max","","Returns the most verbose logging level filter.",1,{"inputs":[],"output":{"name":"loglevelfilter"}}],[11,"to_log_level","","Converts `self` to the equivalent `LogLevel`.",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"args","","The message body.",3,{"inputs":[{"name":"self"}],"output":{"name":"arguments"}}],[11,"metadata","","Metadata about the log directive.",3,{"inputs":[{"name":"self"}],"output":{"name":"logmetadata"}}],[11,"location","","The location of the log directive.",3,{"inputs":[{"name":"self"}],"output":{"name":"loglocation"}}],[11,"level","","The verbosity level of the message.",3,{"inputs":[{"name":"self"}],"output":{"name":"loglevel"}}],[11,"target","","The name of the target of the directive.",3,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"bool"}}],[11,"cmp","","",4,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",4,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"option"}}],[11,"lt","","",4,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"bool"}}],[11,"le","","",4,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"bool"}}],[11,"gt","","",4,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"bool"}}],[11,"ge","","",4,{"inputs":[{"name":"self"},{"name":"logmetadata"}],"output":{"name":"bool"}}],[11,"hash","","",4,null],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"level","","The verbosity level of the message.",4,{"inputs":[{"name":"self"}],"output":{"name":"loglevel"}}],[11,"target","","The name of the target of the directive.",4,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"clone","","",5,{"inputs":[{"name":"self"}],"output":{"name":"loglocation"}}],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",5,{"inputs":[{"name":"self"},{"name":"loglocation"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"self"},{"name":"loglocation"}],"output":{"name":"bool"}}],[11,"cmp","","",5,{"inputs":[{"name":"self"},{"name":"loglocation"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",5,{"inputs":[{"name":"self"},{"name":"loglocation"}],"output":{"name":"option"}}],[11,"lt","","",5,{"inputs":[{"name":"self"},{"name":"loglocation"}],"output":{"name":"bool"}}],[11,"le","","",5,{"inputs":[{"name":"self"},{"name":"loglocation"}],"output":{"name":"bool"}}],[11,"gt","","",5,{"inputs":[{"name":"self"},{"name":"loglocation"}],"output":{"name":"bool"}}],[11,"ge","","",5,{"inputs":[{"name":"self"},{"name":"loglocation"}],"output":{"name":"bool"}}],[11,"hash","","",5,null],[11,"module_path","","The module path of the message.",5,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"file","","The source file containing the message.",5,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"line","","The line containing the message.",5,{"inputs":[{"name":"self"}],"output":{"name":"u32"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"get","","Gets the current maximum log level filter.",6,{"inputs":[{"name":"self"}],"output":{"name":"loglevelfilter"}}],[11,"set","","Sets the maximum log level.",6,{"inputs":[{"name":"self"},{"name":"loglevelfilter"}],"output":null}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",7,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",8,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[14,"log","","The standard logging macro.",null,null],[14,"error","","Logs a message at the error level.",null,null],[14,"warn","","Logs a message at the warn level.",null,null],[14,"info","","Logs a message at the info level.",null,null],[14,"debug","","Logs a message at the debug level.",null,null],[14,"trace","","Logs a message at the trace level.",null,null],[14,"log_enabled","","Determines if a message logged at the specified level in that module will be logged.",null,null]],"paths":[[4,"LogLevel"],[4,"LogLevelFilter"],[8,"Log"],[3,"LogRecord"],[3,"LogMetadata"],[3,"LogLocation"],[3,"MaxLogLevelFilter"],[3,"SetLoggerError"],[3,"ShutdownLoggerError"]]};
searchIndex["netstring"] = {"doc":"","items":[[0,"channel","netstring","",null,null],[3,"Channel","netstring::channel","",null,null],[4,"ChannelError","","",null,null],[13,"ChannelClosed","","",0,null],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"channel"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"map","","",1,{"inputs":[{"name":"r"},{"name":"w"},{"name":"syncsender"},{"name":"usize"},{"name":"f"}],"output":{"name":"self"}}],[11,"new","","",1,{"inputs":[{"name":"r"},{"name":"w"},{"name":"syncsender"},{"name":"usize"}],"output":{"name":"self"}}],[11,"send","","",1,{"inputs":[{"name":"self"},{"name":"s"}],"output":{"name":"result"}}],[11,"flush","","Flushes all pending operations",1,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"send_last","","Sends a last message and consumes the channel",1,{"inputs":[{"name":"self"},{"name":"s"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",0,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"cause","","",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"from","","",0,{"inputs":[{"name":"senderror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"recverror"}],"output":{"name":"self"}}],[8,"ReadNetstring","netstring","",null,null],[10,"read_netstring","","",2,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[8,"WriteNetstring","","",null,null],[10,"write_netstring","","",3,{"inputs":[{"name":"self"},{"name":"s"}],"output":{"name":"result"}}],[10,"flush","","",3,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[8,"Shutdown","","",null,null],[10,"shutdown","","",4,{"inputs":[{"name":"self"},{"name":"shutdownmode"}],"output":{"name":"result"}}]],"paths":[[4,"ChannelError"],[3,"Channel"],[8,"ReadNetstring"],[8,"WriteNetstring"],[8,"Shutdown"]]};
initSearch(searchIndex);