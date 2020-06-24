
provider tide {
	probe startroute(char*, char*, char*);
	probe endroute(char*, char*, int, int);
	probe fire(char*, char*);
};