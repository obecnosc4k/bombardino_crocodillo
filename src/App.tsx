import React, {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import HomeComponents from "./Components/HomePage/HomeComponents.tsx";
import NavbarComponent from "./Components/StyledElement/NavbarComponent.tsx";
import {Box, createTheme, ThemeProvider} from "@mui/material";
import AdminComponent from "./Components/AdminPage/AdminComponent.tsx";
import AllTeacherPages from "./Components/ListTeacher/AllTeacherPages.tsx";
import {appDataDir, cacheDir, configDir} from "@tauri-apps/api/path";

function App() {

    const theme = createTheme({
        components: {
            MuiContainer: {
                styleOverrides: {
                    root: {
                        padding: 0,
                        margin: 0,
                    },
                },
            },
            MuiBox: {
                styleOverrides: {
                    root: {
                        padding: 0,
                        margin: 0,
                    },
                },
            },
            MuiTypography: {
                styleOverrides: {
                    root: {
                        margin: 0,
                    },
                },
            },
        },
    });

    const [isInitialized, setIsInitialized] = useState(false);

    let runs = 0;
    
    async function initalizeDb(){

        if (runs == 1) return;
        runs++;


        let app_data = await appDataDir();
        let cache_dir = await cacheDir();
        let config_dir = await configDir();

        console.log('Paths initialized:', { app_data, cache_dir, config_dir });

        // Logowanie przed wywołaniem invoke
        console.log("Before invoking save_paths");

        await invoke('save_paths', {
            appData: app_data,
            cacheDir: cache_dir,
            configDir: config_dir,
        });

        console.log("After invoking save_paths");

        setIsInitialized(true);
    }

    useEffect(() => {
        initalizeDb();
    }, [])

    if (!isInitialized) {
        return <div>Ładowanie danych...</div>;
    }

    return (
        <ThemeProvider theme={theme}>
            <BrowserRouter>
                <NavbarComponent/>
                <Box sx={{padding: "10px"}}>
                    <Routes>
                        <Route path={"/"} element={<HomeComponents/>}/>
                        <Route path={"/admin"} element={<AdminComponent/>}/>
                        <Route path={"/all"} element={<AllTeacherPages/>}/>
                    </Routes>
                </Box>
            </BrowserRouter>
        </ThemeProvider>

    );
}

export default App;
