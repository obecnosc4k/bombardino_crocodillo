import React, {useState} from 'react';
import {Box, Input} from "@mui/material";
import Teacher from "../../models/teacher.ts";
import Button from "@mui/material/Button";
import {invoke} from "@tauri-apps/api/core";

const AddTeacherComponent = () => {

    // Teacher {
    //     id: number;
    //     name: string,
    //         grades: string, // "6,3,5..."
    //         total_lessions: number,
    //         present: number,
    //         absent: number,
    //         late: number,
    // }
    const [name, setName] = useState("")
    const [grades, setGrades] = useState("")
    const [totalLessions, setTotalLessions] = useState(0)
    const [present, setPresent] = useState(0)
    const [absent, setAbsent] = useState(0)
    const [late, setLate] = useState(0)


    const NewTeacher: Teacher[] = {
        id: 0,
        name: name,
        grades: grades,
        total_lessions: totalLessions,
        present: present,
        absent: absent,
        late: late
    }

    const handleChangeName = (e: React.ChangeEvent<HTMLInputElement>) => {
        setName(e.target.value);
    }
    const handleChangeGrades = (e: React.ChangeEvent<HTMLInputElement>) => {
        setGrades(e.target.value);
    }
    const handleChangeTotalLessions = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTotalLessions(parseInt(e.target.value));
    }
    const handleChangePresent = (e: React.ChangeEvent<HTMLInputElement>) => {
        console.log(typeof parseInt(e.target.value));
        setPresent(parseInt(e.target.value))
    }
    const handleChangeAbsent = (e: React.ChangeEvent<HTMLInputElement>) => {
        console.log(typeof e.target.value);
        setAbsent(parseInt(e.target.value))
    }
    const handleChangeLate = (e: React.ChangeEvent<HTMLInputElement>) => {
        setLate(parseInt(e.target.value))
    }

    const [test, setTest] = useState();
    const SendData =  () => {
        // send data to backend
        console.log(NewTeacher);
        // @ts-ignore
        try {
            const response = invoke('add_new', {teacher: NewTeacher})
            console.log(response);
            setTest(response);
        } catch (error) {
            setTest(error);
            console.log(error)
        }
        // setTest(response);
        // .then((response) => {
        //     console.log(response);
        // })
        // .catch((error) => {
        //     console.error(error);
        // });
    }

    return (
        <Box>
            <Input placeholder={"Name"} sx={{color: "white", margin: "10px"}} onChange={handleChangeName}/>
            <Input placeholder={"Grades"} sx={{color: "white", margin: "10px"}} onChange={handleChangeGrades}/>
            <Input placeholder={"total_lessions"} sx={{color: "white", margin: "10px"}}
                   onChange={handleChangeTotalLessions} type={"number"}/>
            <Input placeholder={"present"} sx={{color: "white", margin: "10px"}} onChange={handleChangePresent}
                   type={"number"}/>
            <Input placeholder={"absent"} sx={{color: "white", margin: "10px"}} onChange={handleChangeAbsent}
                   type={"number"}/>
            <Input placeholder={"late"} sx={{color: "white", margin: "10px"}} onChange={handleChangeLate}
                   type={"number"}/>


            <Button onClick={SendData}>Send</Button>

            <h1>Test : {test}</h1>
        </Box>
    );
};

export default AddTeacherComponent;
