import React from 'react';
import {Box, Input} from "@mui/material";
import Button from "@mui/material/Button";
import AddTeacherComponent from "./AddTeacherComponent.tsx";

const AdminComponent = () => {

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        console.log(event.target.value);
    };
    return (
        <Box>
            {/*<Input onChange={handleChange}/>*/}
            {/*<Button></Button>*/}
            <AddTeacherComponent/>
        </Box>
    );
};

export default AdminComponent;
