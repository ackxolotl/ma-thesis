let addr = unsafe {
    libc::mmap(
        ptr::null_mut(),
        size + HUGE_PAGE_SIZE,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_32BIT,
        -1,
        0,
    )
};

// calculate the huge page size aligned address by rounding up
let aligned_addr = ((addr as isize + HUGE_PAGE_SIZE as isize - 1)
    & -(HUGE_PAGE_SIZE as isize))
    as *mut libc::c_void;

let free_chunk_size = aligned_addr as usize - addr as usize;

// free unneeded pages (i.e. all chunks of the additionally mapped huge page)
unsafe {
    libc::munmap(addr, free_chunk_size);
    libc::munmap(aligned_addr.add(size), HUGE_PAGE_SIZE - free_chunk_size);
}

// finally map huge pages at the huge page size aligned 32 bit address
unsafe {
    libc::mmap(
        aligned_addr as *mut libc::c_void,
        size,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED
            | libc::MAP_ANONYMOUS
            | libc::MAP_HUGETLB
            | MAP_HUGE_2MB
            | libc::MAP_FIXED,
        -1,
        0,
    )
}
