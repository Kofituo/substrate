(function() {var implementors = {
"sc_transaction_pool":[["impl&lt;Block, Client&gt; LocalTransactionPool for <a class=\"struct\" href=\"sc_transaction_pool/struct.BasicPool.html\" title=\"struct sc_transaction_pool::BasicPool\">BasicPool</a>&lt;<a class=\"struct\" href=\"sc_transaction_pool/struct.FullChainApi.html\" title=\"struct sc_transaction_pool::FullChainApi\">FullChainApi</a>&lt;Client, Block&gt;, Block&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: BlockT,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: ProvideRuntimeApi&lt;Block&gt; + <a class=\"trait\" href=\"sc_client_api/client/trait.BlockBackend.html\" title=\"trait sc_client_api::client::BlockBackend\">BlockBackend</a>&lt;Block&gt; + HeaderBackend&lt;Block&gt; + BlockIdTo&lt;Block&gt; + HeaderMetadata&lt;Block, Error = Error&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client::Api: TaggedTransactionQueue&lt;Block&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()