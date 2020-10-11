@Echo OFF

FOR /D /R %%# in (*) DO (
    PUSHD "%%#"
    FOR %%@ in ("a.png") DO (
        Echo Ren: ".\%%~n#\%%@" "%%~n#%%~x@"
        Ren "%%@" "%%~n#%%~x@"
        Move "%%~n#%%~x@*" "..\linzi\"
    )
    POPD
)

Pause&Exit