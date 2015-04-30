initSidebarItems({"struct":[["Batching","A “meta iterator adaptor”. Its closure recives a reference to the iterator and may pick off as many elements as it likes, to produce the next iterator element."],["Dedup","An iterator adaptor that removes duplicates from sections of consecutive identical elements.  If the iterator is sorted, all elements will be unique."],["FnMap","A clonable iterator adaptor to map elementwise from one iterator to another, using a function pointer."],["GroupBy","An iterator adaptor that groups iterator elements. Consecutive elements that map to the same key (“runs”), are returned as the iterator elements."],["Interleave","An iterator adaptor that alternates elements from two iterators until both run out."],["Merge","An iterator adaptor that merges the two base iterators in ascending order. If both base iterators are sorted (ascending), the result is sorted."],["MultiPeek","An iterator adaptor that allows the user to peek at multiple *.next()* values without advancing itself."],["Product","An iterator adaptor that iterates over the cartesian product of the element sets of two iterators **I** and **J**."],["PutBack","An iterator adaptor that allows putting back a single item to the front of the iterator."],["Step","An iterator adaptor that steps a number elements in the base iterator for each iteration."]]});