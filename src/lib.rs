pub use bindings::*;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod bindings;

// Reimplemented Lua C API functions
pub unsafe fn lua_getextraspace(state: *mut lua_State) -> *mut c_void {
    (state as *mut c_void).offset(-(mem::size_of::<*mut c_void>() as isize))
}

pub unsafe fn lua_pcall(
    state: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    msgh: c_int,
) -> c_int {
    lua_pcallk(state, nargs, nresults, msgh, 0, None)
}

pub unsafe fn lua_newuserdata(state: *mut lua_State, s: usize) -> *mut ::std::os::raw::c_void {
    lua_newuserdatauv(state, s, 1)
}

pub unsafe fn lua_getuservalue(state: *mut lua_State, idx: c_int) -> c_int {
    lua_getiuservalue(state, idx, 1)
}

pub unsafe fn lua_setuservalue(state: *mut lua_State, idx: c_int) -> c_int {
    lua_setiuservalue(state, idx, 1)
}

pub unsafe fn lua_tonumber(state: *mut lua_State, idx: c_int) -> lua_Number {
    return lua_tonumberx(state, idx, ptr::null_mut());
}
pub unsafe fn lua_tointeger(state: *mut lua_State, idx: c_int) -> lua_Number {
    return lua_tonumberx(state, idx, ptr::null_mut());
}
pub unsafe fn lua_pop(state: *mut lua_State, n: c_int) {
    lua_settop(state, -(n) - 1);
}
pub unsafe fn lua_isfunction(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_TFUNCTION as i32;
}
pub unsafe fn lua_istable(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_TTABLE as i32;
}
pub unsafe fn lua_islightuserdata(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_TLIGHTUSERDATA as i32;
}
pub unsafe fn lua_isnil(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_TNIL as i32;
}
pub unsafe fn lua_isboolean(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_TBOOLEAN as i32;
}
pub unsafe fn lua_isthread(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_TTHREAD as i32;
}
pub unsafe fn lua_isnone(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_TNONE as i32;
}
pub unsafe fn lua_isnoneornil(state: *mut lua_State, n: c_int) -> bool {
    return lua_type(state, n) == LUA_TNONE as i32;
}

pub unsafe fn lua_pushliteral(state: *mut lua_State, str: *const c_char) -> *const c_char {
    return lua_pushstring(state, str);
}

pub unsafe fn lua_pushglobaltable(state: *mut lua_State) {
    lua_rawgeti(state, LUA_REGISTRYINDEX as i32, LUA_RIDX_GLOBALS as i64);
}

pub unsafe fn lua_newtable(state: *mut lua_State) {
    lua_createtable(state, 0, 0);
}

pub unsafe fn lua_register(state: *mut lua_State, n: *const c_char, f: lua_CFunction) {
    lua_pushcfunction(state, f);
    lua_setglobal(state, n);
}

pub unsafe fn lua_pushcfunction(state: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(state, f, 0);
}

pub unsafe fn lua_tostring(state: *mut lua_State, i: c_int) -> *const c_char {
    return lua_tolstring(state, i, ptr::null_mut());
}

pub unsafe fn lua_insert(state: *mut lua_State, idx: c_int) {
    lua_rotate(state, idx, 1);
}

pub unsafe fn lua_remove(state: *mut lua_State, idx: c_int) {
    lua_rotate(state, idx, -1);
    lua_pop(state, 1);
}

pub unsafe fn lua_replace(state: *mut lua_State, idx: c_int) {
    lua_copy(state, -1, idx);
    lua_pop(state, 1);
}

pub unsafe fn lua_upvalueindex(index: c_int) -> i32 {
    return LUA_REGISTRYINDEX - index;
}

pub unsafe fn lua_call(state: *mut lua_State, nargs: c_int, nresults: c_int) {
    lua_callk(state, nargs, nresults, 0, None)
}

pub unsafe fn lua_yield(state: *mut lua_State, n: c_int) -> c_int {
    lua_yieldk(state, n, 0, None)
}

#[allow(non_snake_case)]
pub unsafe fn luaL_getmetatable(state: *mut lua_State, n: *const i8) -> c_int {
    lua_getfield(state, LUA_REGISTRYINDEX, n)
}

#[allow(non_snake_case)]
pub unsafe fn luaL_loadfile(state: *mut lua_State, f: *const c_char) -> c_int {
    luaL_loadfilex(state, f, std::ptr::null())
}

#[allow(non_snake_case)]
pub unsafe fn luaL_checkstring(state: *mut lua_State, n: c_int) -> *const c_char {
    luaL_checklstring(state, n, std::ptr::null_mut())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn version_test() {
        // Create the new Lua state and open the libaries
        let l = unsafe { luaL_newstate() };
        unsafe { luaL_openlibs(l) };

        // Get the global _VERSION field
        let version = CString::new("_VERSION").expect("version");
        unsafe { lua_getglobal(l, version.as_ptr()) };

        // Convert the _VERSION field into a Rust string
        let version_string_ptr = unsafe { lua_tostring(l, -1) };
        let version_string = unsafe { CStr::from_ptr(version_string_ptr) }
            .to_str()
            .unwrap();

        assert_eq!(version_string, "Lua 5.4")
    }
}
