#pragma D option quiet
tideserver*:::
{
	printf("%s Fired: Method: %s Path: %s Status: %d starttime: %d\n",probefunc, arg0, arg1, arg2, arg3);

	@[pid] = count();
}