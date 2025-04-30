use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

mod readdata;
use readdata::{DataFrame, ColumnVal};

mod winpath;
use winpath::{Graph, find_transitive_win_path};
// import modules

// defines types vertex which is a unsigned interger and list of edges which is a vector of tuples with unsigned integers
type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;


