searchState.loadedDescShard("axstd", 0, "The ArceOS Standard Library\nInspection and manipulation of the process’s environment.\nFilesystem manipulation operations.\nTraits, helpers, and type definitions for core I/O …\nNetworking primitives for TCP/UDP communication.\nOS-specific functionality.\nPrints to the standard output.\nPrints to the standard output, with a newline.\nA module for working with processes.\nUseful synchronization primitives.\nNative threads.\nTemporal quantification.\nReturns the current working directory as a <code>String</code>.\nChanges the current working directory to the specified …\nBlock device\nCharacter device\nDirectory\nA builder used to create directories in various manners.\nEntries returned by the <code>ReadDir</code> iterator.\nFIFO (named pipe)\nAn object providing access to an open file on the …\nRegular file\nA structure representing a type of file with accessors for …\nMetadata information about a file.\nOptions and flags which can be used to configure how a …\nRepresentation of the various permissions on a file.\nIterator over the entries in a directory.\nSocket\nSymbolic link\nSets the option for the append mode.\nReturns the number of blocks allocated to the file, in …\nOpens a file in write-only mode.\nCreates the specified directory with the options …\nSets the option to create a new file, or open it if it …\nCreates a new, empty directory at the provided path.\nRecursively create a directory and all of its parent …\nCreates a new file in read-write mode; error if the file …\nSets the option to create a new file, failing if it …\nReturns the bare file name of this directory entry without …\nReturns the file type for the file that this entry points …\nReturns the file type for this metadata.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if this metadata is for a directory. The …\nReturns <code>true</code> if this metadata is for a regular file. The …\nReturns the size of the file, in bytes, this metadata is …\nGiven a path, query the file system to get information …\nQueries metadata about the underlying file.\nCreates a new set of options with default mode/security …\nCreates a blank new set of options ready for configuration.\nAttempts to open a file in read-only mode.\nOpens a file at <code>path</code> with the options specified by <code>self</code>.\nReturns a new OpenOptions object.\nReturns the full path to the file that this entry …\nReturns the permissions of the file this metadata is for.\nRead the entire contents of a file into a bytes vector.\nSets the option for read access.\nReturns an iterator over the entries within a directory.\nRead the entire contents of a file into a string.\nIndicates that directories should be created recursively, …\nRemoves an empty directory.\nRemoves a file from the filesystem.\nRename a file or directory to a new name. Delete the …\nTruncates or extends the underlying file, updating the …\nReturns the total size of this file in bytes.\nSets the option for truncating a previous file.\nWrite a slice as the entire contents of a file.\nSets the option for write access.\nA socket address could not be bound because the address is …\nTryAgain\nAn entity already exists, often a file.\nBad address.\nBad internal state.\nA <code>BufRead</code> is a type of <code>Read</code>er which has an internal …\nThe <code>BufReader&lt;R&gt;</code> struct adds buffering to any reader.\nDevice or resource busy\nThe connection was refused by the remote server,\nThe connection was reset by the remote server.\nSets the offset to the current position plus the specified …\nA non-empty directory was specified where an empty …\nSets the offset to the size of this object plus the …\nContains the error value\nThe error type used by ArceOS.\nSyscall interrupted by a caught signal\nData not valid for the operation were encountered.\nInvalid parameter/argument.\nInput/output error.\nThe filesystem object is, unexpectedly, a directory.\nNot enough space/cannot allocate memory.\nA filesystem object is, unexpectedly, not a directory.\nThe network operation failed because it was not connected …\nThe requested entity is not found.\nContains the success value\nThe operation lacked the necessary privileges to complete.\nThe <code>Read</code> trait allows for reading bytes from a source.\nDevice or resource is busy.\nA specialized <code>Result</code> type for I/O operations.\nThe <code>Seek</code> trait provides a cursor which can be moved within …\nEnumeration of possible methods to seek within an I/O …\nSets the offset to the provided number of bytes.\nA handle to the standard input stream of a process.\nA locked reference to the <code>Stdin</code> handle.\nA handle to the global standard output stream of the …\nA locked reference to the <code>Stdout</code> handle.\nThe underlying storage (typically, a filesystem) is full.\nSyscall timed out\nAn error returned when an operation could not be completed …\nThis operation is unsupported or unimplemented.\nThe operation needs to block to complete, but the blocking …\nA trait for objects which are byte-oriented sinks.\nAn error returned when an operation could not be completed …\nReturns the error description.\nReturns a reference to the internally buffered data.\nReturns the number of bytes the internal buffer can hold …\nReturns the error code value in <code>i32</code>.\nTells this buffer that <code>amt</code> bytes have been consumed from …\nReturns the contents of the internal buffer, filling it …\nFlush this output stream, ensuring that all intermediately …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets a mutable reference to the underlying reader.\nGets a reference to the underlying reader.\nCheck if the underlying <code>Read</code> has any data left to be read.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUnwraps this <code>BufReader&lt;R&gt;</code>, returning the underlying reader.\nLocks this handle to the standard input stream, returning …\nLocks this handle to the standard output stream, returning …\nCreates a new <code>BufReader&lt;R&gt;</code> with a default buffer capacity …\nThe I/O Prelude.\nPull some bytes from this source into the specified …\nRead the exact number of bytes required to fill <code>buf</code>.\nRead the exact number of bytes required to fill <code>buf</code>.\nLocks this handle and reads a line of input, appending it …\nRead all bytes until a newline (the <code>0xA</code> byte) is reached, …\nRead all bytes until EOF in this source, placing them into …\nRead all bytes until EOF in this source, appending them to …\nRead all bytes into <code>buf</code> until the delimiter <code>byte</code> or EOF is …\nRewind to the beginning of a stream.\nSeek to an offset, in bytes, in a stream.\nConstructs a new handle to the standard input of the …\nConstructs a new handle to the standard output of the …\nReturns the current seek position from the start of the …\nWrite a buffer into this writer, returning how many bytes …\nAttempts to write an entire buffer into this writer.\nWrites a formatted string into this writer, returning any …\nA <code>BufRead</code> is a type of <code>Read</code>er which has an internal …\nThe <code>Read</code> trait allows for reading bytes from a source.\nThe <code>Seek</code> trait provides a cursor which can be moved within …\nA trait for objects which are byte-oriented sinks.\nTells this buffer that <code>amt</code> bytes have been consumed from …\nReturns the contents of the internal buffer, filling it …\nFlush this output stream, ensuring that all intermediately …\nCheck if the underlying <code>Read</code> has any data left to be read.\nPull some bytes from this source into the specified …\nRead the exact number of bytes required to fill <code>buf</code>.\nRead the exact number of bytes required to fill <code>buf</code>.\nRead all bytes until a newline (the <code>0xA</code> byte) is reached, …\nRead all bytes until EOF in this source, placing them into …\nRead all bytes until EOF in this source, appending them to …\nRead all bytes into <code>buf</code> until the delimiter <code>byte</code> or EOF is …\nRewind to the beginning of a stream.\nSeek to an offset, in bytes, in a stream.\nReturns the current seek position from the start of the …\nWrite a buffer into this writer, returning how many bytes …\nAttempts to write an entire buffer into this writer.\nWrites a formatted string into this writer, returning any …\nThe size of an IPv4 address in bits.\nThe size of an IPv6 address in bits.\nAn IPv4 address representing the broadcast address: …\nAn IP address, either IPv4 or IPv6.\nAn IPv4 address.\nAn IPv6 address.\nReturned iterator over socket addresses which this type …\nAn IPv4 address with the address pointing to localhost: …\nAn IPv6 address representing localhost: <code>::1</code>.\nAn internet socket address, either IPv4 or IPv6.\nAn IPv4 socket address.\nAn IPv6 socket address.\nA TCP socket server, listening for connections.\nA TCP stream between a local and a remote socket.\nA trait for objects which can be converted or resolved to …\nAn IPv4 address representing an unspecified address: …\nAn IPv6 address representing the unspecified address: <code>::</code>\nA UDP socket.\nAn IPv4 address.\nAn IPv4 socket address.\nAn IPv6 address.\nAn IPv6 socket address.\nAccept a new incoming connection from this listener.\nCreates a new <code>TcpListener</code> which will be bound to the …\nCreates a UDP socket from the given address.\nOpens a TCP connection to a remote host.\nConnects this UDP socket to a remote address, allowing the …\nReturns the flow information associated with this address.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates an <code>IpAddr::V6</code> from an eight element 16-bit array.\nCreates an <code>IpAddr::V6</code> from a sixteen element byte array.\nCopies this address to a new <code>IpAddr::V6</code>.\nCopies this address to a new <code>IpAddr::V4</code>.\nCreates an <code>IpAddr::V4</code> from a four element byte array.\nUses <code>Ipv4Addr::from_bits</code> to convert a host byte order <code>u32</code> …\nReturns the argument unchanged.\nCreates an <code>Ipv4Addr</code> from a four element byte array.\nCreates an <code>Ipv6Addr</code> from an eight element 16-bit array.\nReturns the argument unchanged.\nUses <code>Ipv6Addr::from_bits</code> to convert a host byte order <code>u128</code> …\nCreates an <code>Ipv6Addr</code> from a sixteen element byte array.\nConverts a tuple struct (Into&lt;<code>IpAddr</code>&gt;, <code>u16</code>) into a …\nConverts a <code>SocketAddrV4</code> into a <code>SocketAddr::V4</code>.\nConverts a <code>SocketAddrV6</code> into a <code>SocketAddr::V6</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts a native byte order <code>u32</code> into an IPv4 address.\nConverts a native byte order <code>u128</code> into an IPv6 address.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the IP address associated with this socket address.\nReturns the IP address associated with this socket address.\nReturns the IP address associated with this socket address.\nReturns <code>true</code> if this address is in a range designated for …\nReturns <code>true</code> if this address part of the <code>198.18.0.0/15</code> …\nReturns <code>true</code> if this is an address reserved for …\nReturns <code>true</code> if this is a broadcast address (…\nReturns <code>true</code> if this address is in a range designated for …\nReturns <code>true</code> if this address is in a range designated for …\nReturns <code>true</code> if this is an address reserved for …\nReturns <code>true</code> if the address appears to be globally …\nReturns <code>true</code> if the address appears to be globally …\nReturns <code>true</code> if the address appears to be globally …\nReturns <code>true</code> if this address is an <code>IPv4</code> address, and <code>false</code> …\nReturns <code>true</code> if the IP address in this <code>SocketAddr</code> is an …\nReturns <code>true</code> if the address is an IPv4-mapped address (…\nReturns <code>true</code> if this address is an <code>IPv6</code> address, and <code>false</code> …\nReturns <code>true</code> if the IP address in this <code>SocketAddr</code> is an …\nReturns <code>true</code> if the address is link-local (<code>169.254.0.0/16</code>).\nReturns <code>true</code> if this is a loopback address.\nReturns <code>true</code> if this is a loopback address (<code>127.0.0.0/8</code>).\nReturns <code>true</code> if this is the loopback address (<code>::1</code>), as …\nReturns <code>true</code> if this is a multicast address.\nReturns <code>true</code> if this is a multicast address (<code>224.0.0.0/4</code>).\nReturns <code>true</code> if this is a multicast address (<code>ff00::/8</code>).\nReturns <code>true</code> if this is a private address.\nReturns <code>true</code> if this address is reserved by IANA for …\nReturns <code>true</code> if this address is part of the Shared Address …\nReturns <code>true</code> if this is a unicast address, as defined by …\nReturns <code>true</code> if the address is a globally routable unicast …\nReturns <code>true</code> if the address is a unicast address with …\nReturns <code>true</code> if this is a unique local address (<code>fc00::/7</code>).\nReturns <code>true</code> for the special ‘unspecified’ address.\nReturns <code>true</code> for the special ‘unspecified’ address (…\nReturns <code>true</code> for the special ‘unspecified’ address (<code>::</code>…\nReturns the socket address of the local half of this TCP …\nReturns the local socket address of this listener.\nReturns the socket address that this socket was created …\nReturns the address’s multicast scope if the address is …\nCreates a new IPv4 address from four eight-bit octets.\nCreates a new IPv6 address from eight 16-bit segments.\nCreates a new socket address from an IP address and a port …\nCreates a new socket address from an <code>IPv4</code> address and a …\nCreates a new socket address from an <code>IPv6</code> address, a …\nReturns the four eight-bit integers that make up this …\nReturns the sixteen eight-bit integers the IPv6 address …\nParse an IP address from a slice of bytes.\nParse an IPv4 address from a slice of bytes.\nParse an IPv6 address from a slice of bytes.\nParse a socket address from a slice of bytes.\nParse an IPv4 socket address from a slice of bytes.\nParse an IPv6 socket address from a slice of bytes.\nReceives a single datagram message on the socket, without …\nReturns the socket address of the remote peer of this TCP …\nReturns the socket address of the remote peer this socket …\nReturns the port number associated with this socket …\nReturns the port number associated with this socket …\nReturns the port number associated with this socket …\nReceives a single datagram message on the socket from the …\nReceives a single datagram message on the socket. On …\nReturns the scope ID associated with this address.\nReturns the eight 16-bit segments that make up this …\nSends data on the socket to the remote address to which it …\nSends data on the socket to the given address. On success, …\nChanges the flow information associated with this socket …\nChanges the IP address associated with this socket address.\nChanges the IP address associated with this socket address.\nChanges the IP address associated with this socket address.\nChanges the port number associated with this socket …\nChanges the port number associated with this socket …\nChanges the port number associated with this socket …\nChanges the scope ID associated with this socket address.\nShuts down the connection.\nConverts an IPv4 address into a <code>u32</code> representation using …\nConverts an IPv6 address into a <code>u128</code> representation using …\nConverts this address to an <code>IpAddr::V4</code> if it is an …\nConverts this address to an <code>IpAddr::V4</code> if it is an …\nConverts this address to an <code>IPv4</code> address if it is either …\nConverts this address to an <code>IPv4</code> address if it’s an …\nConverts this address to an IPv4-compatible <code>IPv6</code> address.\nConverts this address to an IPv4-mapped <code>IPv6</code> address.\nConverts this object to an iterator of resolved <code>SocketAddr</code>…\nArceOS-specific definitions.\nShutdown the whole system.\nA mutual exclusion primitive useful for protecting shared …\nA guard that provides mutable data access.\nThe dropping of the <code>MutexGuard</code> will release the lock it …\nForce unlock the <code>Mutex</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns a mutable reference to the underlying data.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes this <code>Mutex</code> and unwraps the underlying data.\nReturns <code>true</code> if the lock is currently held.\nLocks the <code>Mutex</code> and returns a guard that permits access to …\nCreates a new <code>Mutex</code> wrapping the supplied data.\nTry to lock this <code>Mutex</code>, returning a lock guard if …\nThread factory, which can be used in order to configure …\nAn owned permission to join on a thread (block on its …\nA handle to a thread.\nA unique identifier for a running thread.\nThis returns a numeric identifier for the thread …\nGets a handle to the thread that invokes it.\nExits the current thread.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets the thread’s unique identifier.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWaits for the associated thread to finish.\nNames the thread-to-be.\nGenerates the base configuration for spawning a thread, …\nCurrent thread is going to sleep for the given duration.\nCurrent thread is going to sleep, it will be woken up at …\nSpawns a new thread, returning a <code>JoinHandle</code> for it.\nSpawns a new thread by taking ownership of the <code>Builder</code>, …\nSets the size of the stack (in bytes) for the new thread.\nExtracts a handle to the underlying thread.\nCurrent thread gives up the CPU time voluntarily, and …\nA <code>Duration</code> type to represent a span of time, typically …\nA measurement of a monotonically nondecreasing clock. …\nThe maximum duration.\nThe duration of one microsecond.\nThe duration of one millisecond.\nThe duration of one nanosecond.\nThe duration of one second.\nA duration of zero time.\nComputes the absolute difference between <code>self</code> and <code>other</code>.\nPanics\nReturns the total number of whole microseconds contained …\nReturns the total number of whole milliseconds contained …\nReturns the number of milliseconds contained by this …\nReturns the number of milliseconds contained by this …\nReturns the total number of nanoseconds contained by this …\nReturns the number of <em>whole</em> seconds contained by this …\nReturns the number of seconds contained by this <code>Duration</code> …\nReturns the number of seconds contained by this <code>Duration</code> …\nReturns <code>Some(t)</code> where <code>t</code> is the time <code>self + duration</code> if <code>t</code> …\nChecked <code>Duration</code> addition. Computes <code>self + other</code>, …\nChecked <code>Duration</code> division. Computes <code>self / other</code>, …\nChecked <code>Duration</code> multiplication. Computes <code>self * other</code>, …\nReturns <code>Some(t)</code> where <code>t</code> is the time <code>self - duration</code> if <code>t</code> …\nChecked <code>Duration</code> subtraction. Computes <code>self - other</code>, …\nDivide <code>Duration</code> by <code>Duration</code> and return <code>f32</code>.\nDivide <code>Duration</code> by <code>Duration</code> and return <code>f64</code>.\nDivide <code>Duration</code> by <code>f32</code>.\nDivide <code>Duration</code> by <code>f64</code>.\nReturns the amount of time elapsed from another instant to …\nReturns the amount of time elapsed since this instant was …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a new <code>Duration</code> from the specified number of days.\nCreates a new <code>Duration</code> from the specified number of hours.\nCreates a new <code>Duration</code> from the specified number of …\nCreates a new <code>Duration</code> from the specified number of …\nCreates a new <code>Duration</code> from the specified number of …\nCreates a new <code>Duration</code> from the specified number of …\nCreates a new <code>Duration</code> from the specified number of whole …\nCreates a new <code>Duration</code> from the specified number of …\nCreates a new <code>Duration</code> from the specified number of …\nCreates a new <code>Duration</code> from the specified number of weeks.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if this <code>Duration</code> spans no time.\nMultiplies <code>Duration</code> by <code>f32</code>.\nMultiplies <code>Duration</code> by <code>f64</code>.\nCreates a new <code>Duration</code> from the specified number of whole …\nReturns an instant corresponding to “now”.\nSaturating <code>Duration</code> addition. Computes <code>self + other</code>, …\nSaturating <code>Duration</code> multiplication. Computes <code>self * other</code>, …\nSaturating <code>Duration</code> subtraction. Computes <code>self - other</code>, …\nReturns the amount of time elapsed from another instant to …\nReturns the fractional part of this <code>Duration</code>, in whole …\nReturns the fractional part of this <code>Duration</code>, in whole …\nReturns the fractional part of this <code>Duration</code>, in …\nThe checked version of <code>from_secs_f32</code>.\nThe checked version of <code>from_secs_f64</code>.")