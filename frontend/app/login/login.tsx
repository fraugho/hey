"use client"

import { useState, FormEvent } from 'react'
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

export function LoginCard(): JSX.Element {
    const [loading, setLoading] = useState<boolean>(false)
    const [error, setError] = useState<string>('')

    const handleLogin = async (e: FormEvent<HTMLFormElement>): Promise<void> => {
        e.preventDefault()
        setError('')
        setLoading(true)

        const formData = new FormData(e.currentTarget)
        
        try {
            const response = await fetch('http://localhost:8080/api/login', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                },
                body: formData,
                credentials: 'include',
            })

            if (response.ok) {
                const data = await response.json()
                // Handle successful login here
                console.log('Login successful:', data)
            } else {
                const errorData = await response.json()
                setError(errorData.message || 'Login failed. Please try again.')
            }
        } catch (err) {
            console.error('Login error:', err)
            setError('Network error. Please try again.')
        } finally {
            setLoading(false)
        }
    }

    const handleSignup = async (e: FormEvent<HTMLFormElement>): Promise<void> => {
        e.preventDefault()
        setError('')
        setLoading(true)

        const formData = new FormData(e.currentTarget)

        try {
            const response = await fetch('http://localhost:8080/api/signup', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                },
                body: formData,
                credentials: 'include',
            })

            if (response.ok) {
                const data = await response.json()
                console.log('Signup successful:', data)
                // Handle successful signup here
            } else {
                const errorData = await response.json()
                setError(errorData.message || 'Signup failed. Please try again.')
            }
        } catch (err) {
            console.error('Signup error:', err)
            setError('Network error. Please try again.')
        } finally {
            setLoading(false)
        }
    }

    return (
        <Tabs defaultValue="Login" className="w-[400px]">
            <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="Login">Login</TabsTrigger>
                <TabsTrigger value="Create Account">Create Account</TabsTrigger>
            </TabsList>
            
            {error && (
                <div className="mt-4 text-sm text-red-500 text-center">
                    {error}
                </div>
            )}
            
            <TabsContent value="Login">
                <Card>
                    <CardHeader>
                        <CardTitle className="text-center">Login</CardTitle>
                        <CardDescription className="text-center">
                            Login To Your Account
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-2">
                        <form onSubmit={handleLogin}>
                            <div className="space-y-1">
                                <Label htmlFor="login_username">Username</Label>
                                <Input 
                                    id="login_username" 
                                    name="username" 
                                    type="text"
                                    placeholder="Username"
                                    required 
                                    autoComplete="username"
                                />
                            </div>
                            <div className="space-y-1 pt-3">
                                <Label htmlFor="login_password">Password</Label>
                                <Input 
                                    id="login_password" 
                                    name="password" 
                                    type="password" 
                                    placeholder="Password"
                                    required 
                                    autoComplete="current-password"
                                />
                            </div>
                            <Button 
                                type="submit" 
                                className="w-full mt-3"
                                disabled={loading}
                                aria-busy={loading}
                            >
                                {loading ? 'Signing in...' : 'Sign In'}
                            </Button>
                        </form>
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
                        <form onSubmit={handleSignup}>
                            <div className="space-y-1">
                                <Label htmlFor="create_username">Username</Label>
                                <Input 
                                    id="create_username" 
                                    name="username" 
                                    type="text"
                                    placeholder="Username"
                                    required 
                                    autoComplete="username"
                                />
                            </div>
                            <div className="space-y-1 pt-3">
                                <Label htmlFor="create_password">Password</Label>
                                <Input 
                                    id="create_password" 
                                    name="password" 
                                    type="password" 
                                    placeholder="Password"
                                    required 
                                    autoComplete="new-password"
                                />
                            </div>
                            <Button 
                                type="submit" 
                                className="w-full mt-3"
                                disabled={loading}
                                aria-busy={loading}
                            >
                                {loading ? 'Creating Account...' : 'Sign Up'}
                            </Button>
                        </form>
                    </CardContent>
                </Card>
            </TabsContent>
        </Tabs>
    )
}

export default LoginCard;
