@Echo OFF

FOR /D /R %%# in (*) DO (
    PUSHD "%%#"
    FOR %%@ in ("path*.png") DO (
        Echo Ren: ".\%%~n#\%%@" "%%~n#%%~nx@"
        Ren "%%@" "%%~n#%%~nx@"
        Move "%%~n#%%~nx@*" "..\grau_prua_yr\"
    )
    POPD
)

Pause&Exit