import React from 'react';
import {teachersMockData} from "../../MokaData.ts";
import {Box} from "@mui/material";

const ListTeacherComponent = () => {
    console.log(teachersMockData);
    return (
        <Box display={"grid"} gridTemplateColumns={"repeat(3, 1fr)"} gap={2} sx={{margin: "10px", padding: "10px"}}>
            {
                teachersMockData.map((teacher, index) => (
                    <Box key={index} sx={{padding: "10px", margin: "10px", border: "1px solid black", borderRadius: "8px", display: "flex", flexDirection: "column", alignItems: "center"}}>
                        <h2>{teacher.fullName}</h2>
                        <h3>Subject: {teacher.subject}</h3>
                    </Box>
                ))
            }
        </Box>
    );
};

export default ListTeacherComponent;
