#ifndef TIDETRACE_H_   /* Include guard */
#define TIDETRACE_H_

void startroute(char* method, char* path, int id, char* headers);  
void endroute(char* method, char* path, int id, int status, char* headers);
void fire(char* tag, char* data);

#endif // TIDETRACE_H_