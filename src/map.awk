BEGIN     { print "digraph map {"; }
/^[ \t]*\(/     { outputEdges(); delete a; }
/^[ \t]/  { gsub(/"/, "", $3); gsub(/,/, "", $3); a[$1] = $3; }
END       { outputEdges(); print "}"; }
function outputEdges()
{
   outputEdge(a["location"], a["destination"], "");
   outputEdge(a["location"], a["prospect"], " [style=dashed]");
}

function outputEdge(from, to, style)
{
   if (from && to) print "\t" from " -> " to style;
}
