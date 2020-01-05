void* malloc(int size) {
   static unsigned int cur_ptr = 0x00600000;
    void* ptr;
    if ((cur_ptr + (unsigned int) size) > 0x00900000)
        return 0;
    
    ptr = (void*) cur_ptr;
    cur_ptr += (unsigned int) size;
    return ptr;
}
void free(void* ptr) {
}
void bz_internal_error ( int errcode ) {
       while(1);
}