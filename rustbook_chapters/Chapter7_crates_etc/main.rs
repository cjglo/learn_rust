fn main()
{
	// Chapter is mostly read only, but it is worth noting:


	/*
	1. Build module tree yourself, rust only looks in same direcotry when sees "mod"
		a) this means to get into a folder, must build folder with "mod {name}" and then add mod.rs to import files from the directory


	2. Cool note about public vs package private etc!:
		a) so a consiquence of 1a is that the permissions get sorta "cut off" with each directory/mod.rs pair, since the higher up files can only import that mod.rs folder

		this means that a pub module in a subdirectory is not accessible except through the mod.rs, which won't allow use unless declared pub

		aka that public module is sorta "package private" it is only public to those at or below it


		^ The above could be wrong, but so far this is how I understand it to be






	*/
}