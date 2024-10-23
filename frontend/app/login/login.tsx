import { Button } from "@/components/ui/button"
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
    Tabs,
    TabsContent,
    TabsList,
    TabsTrigger,
} from "@/components/ui/tabs"
import "@/app/globals.css";

export function LoginCard() {
    return (
        <Tabs defaultValue="Login" className="w-[400px]">
            <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="Login">Login</TabsTrigger>
                <TabsTrigger value="Create Account">Create Account</TabsTrigger>
            </TabsList>
            <TabsContent value="Login">
                <Card>
                    <CardHeader>
                        <CardTitle className="text-center">Login</CardTitle>
                        <CardDescription className="text-center">
                            Login To Your Account
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-2">
                        <form>
                            <div className="space-y-1">
                                <Label htmlFor="login_username">Username</Label>
                                <Input id="login_username" name="username" placeholder="Username" />
                            </div>
                            <div className="space-y-1 pt-3">
                                <Label htmlFor="login_password">Password</Label>
                                <Input id="login_password" name="password" type="password" placeholder="Password" />
                            </div>
                            <Button type="submit" className="w-full mt-3">Sign In</Button>
                        </form>
                        <p className="text-center pt-4">&quot;thing&quot;</p>
                    </CardContent>
                </Card>
            </TabsContent>
            <TabsContent value="Create Account">
                <Card>
                    <CardHeader>
                        <CardTitle className="text-center">Create Login</CardTitle>
                        <CardDescription className="text-center">
                            Create An Account
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-2">
                        <form>
                            <div className="space-y-1">
                                <Label htmlFor="create_username">Username</Label>
                                <Input id="create_username" name="username" placeholder="Username" />
                            </div>
                            <div className="space-y-1 pt-3">
                                <Label htmlFor="create_password">Password</Label>
                                <Input id="create_password" name="password" type="password" placeholder="Password" />
                            </div>
                            <Button type="submit" className="w-full mt-3">Sign Up</Button>
                        </form>
                        <p className="text-center pt-4">&quot;thing2&quot;</p>
                    </CardContent>
                </Card>
            </TabsContent>
        </Tabs>
    )
}
