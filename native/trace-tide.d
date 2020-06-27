
provider tide {
	probe startroute(char*, char*, int, char*);
	probe endroute(char*, char*, int, int, char*);
	probe fire(char*, char*);
};