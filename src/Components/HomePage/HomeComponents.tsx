import React, {useEffect, useState} from 'react';
import {Box} from "@mui/material";
import ListTeacherComponent from "../ListTeacher/ListTeacherComponent.tsx";
import {invoke} from "@tauri-apps/api/core";
import Teacher from "../../models/teacher.ts";

const HomeComponents = () => {

        const [teachers, setTeachers] = useState<Teacher[]>([]);

        useEffect(() => {
            invoke<Teacher[]>('get_all')
                .then((data) => {
                    setTeachers(data);
                })
                .catch((err) => {
                    console.error('Błąd przy pobieraniu nauczycieli:', err);
                });
        }, []);

    console.log(teachers);
    const today = new Date().toLocaleDateString();
    return (
        <Box sx={{margin: -1,  width: "100vw", height: "100vh", display: "flex", flexDirection:"column", alignItems: "center"}}>
            <Box sx={{margin: "10px ",padding: "10px", borderRadius: "8px", boxShadow: 3, display: "flex", flexDirection:"column", justifyContent: "center"}}>
                <h1>Dziś mamy: {today} </h1>
                {/*<ListTeacherComponent/>*/}
            </Box>
        </Box>
    );
};

export default HomeComponents;
