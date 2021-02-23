(function() {var implementors = {};
implementors["ascii"] = [{"text":"impl FromIterator&lt;AsciiChar&gt; for AsciiString","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FromIterator&lt;&amp;'a AsciiStr&gt; for AsciiString","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FromIterator&lt;Cow&lt;'a, AsciiStr&gt;&gt; for AsciiString","synthetic":false,"types":[]}];
implementors["bit_set"] = [{"text":"impl&lt;B:&nbsp;BitBlock&gt; FromIterator&lt;usize&gt; for BitSet&lt;B&gt;","synthetic":false,"types":[]}];
implementors["bit_vec"] = [{"text":"impl&lt;B:&nbsp;BitBlock&gt; FromIterator&lt;bool&gt; for BitVec&lt;B&gt;","synthetic":false,"types":[]}];
implementors["bstr"] = [{"text":"impl FromIterator&lt;char&gt; for BString","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;u8&gt; for BString","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FromIterator&lt;&amp;'a str&gt; for BString","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FromIterator&lt;&amp;'a [u8]&gt; for BString","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FromIterator&lt;&amp;'a BStr&gt; for BString","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;BString&gt; for BString","synthetic":false,"types":[]}];
implementors["camino"] = [{"text":"impl&lt;P:&nbsp;AsRef&lt;Utf8Path&gt;&gt; FromIterator&lt;P&gt; for Utf8PathBuf","synthetic":false,"types":[]}];
implementors["crossbeam_deque"] = [{"text":"impl&lt;T&gt; FromIterator&lt;Steal&lt;T&gt;&gt; for Steal&lt;T&gt;","synthetic":false,"types":[]}];
implementors["fixedbitset"] = [{"text":"impl FromIterator&lt;usize&gt; for FixedBitSet","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;K, V, S&gt; FromIterator&lt;(K, V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FromIterator&lt;T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;K, V, S&gt; FromIterator&lt;(K, V)&gt; for IndexMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FromIterator&lt;T&gt; for IndexSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["linked_hash_map"] = [{"text":"impl&lt;K:&nbsp;Hash + Eq, V, S:&nbsp;BuildHasher + Default&gt; FromIterator&lt;(K, V)&gt; for LinkedHashMap&lt;K, V, S&gt;","synthetic":false,"types":[]}];
implementors["nested"] = [{"text":"impl&lt;T:&nbsp;Collection, A:&nbsp;AsRef&lt;Item&lt;T&gt;&gt;&gt; FromIterator&lt;A&gt; for Nested&lt;T&gt;","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl FromIterator&lt;AtFlags&gt; for AtFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;OFlag&gt; for OFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SealFlag&gt; for SealFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SpliceFFlags&gt; for SpliceFFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;FallocateFlags&gt; for FallocateFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ModuleInitFlags&gt; for ModuleInitFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;DeleteModuleFlags&gt; for DeleteModuleFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MntFlags&gt; for MntFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MQ_OFlag&gt; for MQ_OFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;InterfaceFlags&gt; for InterfaceFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;PollFlags&gt; for PollFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;CloneFlags&gt; for CloneFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;EpollFlags&gt; for EpollFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;EpollCreateFlags&gt; for EpollCreateFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;EfdFlags&gt; for EfdFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MemFdCreateFlag&gt; for MemFdCreateFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ProtFlags&gt; for ProtFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MapFlags&gt; for MapFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MlockAllFlags&gt; for MlockAllFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Options&gt; for Options","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;QuotaValidFlags&gt; for QuotaValidFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SaFlags&gt; for SaFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SfdFlags&gt; for SfdFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SockFlag&gt; for SockFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;MsgFlags&gt; for MsgFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SFlag&gt; for SFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Mode&gt; for Mode","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;FsFlags&gt; for FsFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;InputFlags&gt; for InputFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;OutputFlags&gt; for OutputFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ControlFlags&gt; for ControlFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;LocalFlags&gt; for LocalFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;WaitPidFlag&gt; for WaitPidFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;AddWatchFlags&gt; for AddWatchFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;InitFlags&gt; for InitFlags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;AccessFlags&gt; for AccessFlags","synthetic":false,"types":[]}];
implementors["petgraph"] = [{"text":"impl&lt;N, E, Ty, Item&gt; FromIterator&lt;Item&gt; for GraphMap&lt;N, E, Ty&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Item: IntoWeightedEdge&lt;E, NodeId = N&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: NodeTrait,<br>&nbsp;&nbsp;&nbsp;&nbsp;Ty: EdgeType,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl FromIterator&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl FromIterator&lt;(String, Value)&gt; for Map&lt;String, Value&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;Value&gt;&gt; FromIterator&lt;T&gt; for Value","synthetic":false,"types":[]},{"text":"impl&lt;K:&nbsp;Into&lt;String&gt;, V:&nbsp;Into&lt;Value&gt;&gt; FromIterator&lt;(K, V)&gt; for Value","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; FromIterator&lt;&lt;A as Array&gt;::Item&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;T, P&gt; FromIterator&lt;T&gt; for Punctuated&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, P&gt; FromIterator&lt;Pair&lt;T, P&gt;&gt; for Punctuated&lt;T, P&gt;","synthetic":false,"types":[]}];
implementors["toml"] = [{"text":"impl FromIterator&lt;(String, Value)&gt; for Map&lt;String, Value&gt;","synthetic":false,"types":[]}];
implementors["toml_edit"] = [{"text":"impl&lt;V:&nbsp;Into&lt;Value&gt;&gt; FromIterator&lt;V&gt; for Value","synthetic":false,"types":[]},{"text":"impl&lt;'k, K:&nbsp;Into&lt;&amp;'k Key&gt;, V:&nbsp;Into&lt;Value&gt;&gt; FromIterator&lt;(K, V)&gt; for Value","synthetic":false,"types":[]}];
implementors["vec_map"] = [{"text":"impl&lt;V&gt; FromIterator&lt;(usize, V)&gt; for VecMap&lt;V&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()