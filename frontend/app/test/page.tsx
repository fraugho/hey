import { Dashboard } from "./dashboard";
export default function Page() {
    return (
        <body className="overflow-hidden flex justify-center items-center min-h-screen bg-black">
            <div className="flex flex-col justify-center items-center min-h-screen">
                <Dashboard />
            </div>
        </body>
    );
}

