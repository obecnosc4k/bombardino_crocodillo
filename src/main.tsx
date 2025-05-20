import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import {Box} from "@mui/material";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <Box sx={{margin: -1,  width: "100vw", height: "100vh"}}>
            <App/>
        </Box>
    </React.StrictMode>
);
