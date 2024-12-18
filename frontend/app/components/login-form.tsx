import { cn } from "~/lib/utils";
import { useState } from "react";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import { useNavigate } from "@remix-run/react";
import { useBaseUrl } from "~/hooks/useBaseUrl";

export function LoginForm({
  className,
  ...props
}: React.ComponentPropsWithoutRef<"div">) {
  const baseUrl = useBaseUrl();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMessage("");
    navigate("/home");

    // try {
    //   const response = await fetch(`${baseUrl}/api/public/login`, {
    //     method: "POST",
    //     headers: {
    //       "Content-Type": "application/json",
    //     },
    //     body: JSON.stringify({ username: username, password }),
    //   });

    //   if (response.ok) {
    //     // Redirect to /home if login is successful
    //     navigate("/home");
    //   } else {
    //     const errorData = await response.json().catch(() => ({}));
    //     if (response.status === 401) {
    //       setErrorMessage("Unauthorized: Invalid username or password.");
    //     } else if (response.status === 422) {
    //       setErrorMessage("Validation Error: " + (errorData.error || "Invalid input."));
    //     } else {
    //       setErrorMessage("An unexpected error occurred. Please try again.");
    //     }
    //   }
    // } catch (error) {
    //   setErrorMessage("A network error occurred. Please try again later.");
    // }
  };

  return (
    <div className={cn("flex flex-col gap-6", className)} {...props}>
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">Login</CardTitle>
          <CardDescription>
            Enter your credentials below to login to your account.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit}>
            <div className="flex flex-col gap-6">
              <div className="grid gap-2">
                <Label htmlFor="username">Username</Label>
                <Input
                  id="username"
                  type="text"
                  placeholder="username"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  required
                />
              </div>
              <div className="grid gap-2">
                <Label htmlFor="password">Password</Label>
                <Input
                  id="password"
                  type="password"
                  placeholder="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                />
              </div>
              {errorMessage && (
                <div className="text-red-500 text-sm text-center">{errorMessage}</div>
              )}
              <Button type="submit" className="w-full">
                Login
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
