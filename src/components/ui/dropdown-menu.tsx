import { useState, useRef, useEffect, ReactNode } from "react"
import { Button } from "./button"

interface DropdownMenuProps {
  children: ReactNode
  trigger: ReactNode
  align?: "start" | "center" | "end"
  className?: string
}

interface DropdownMenuTriggerProps {
  children: ReactNode
  asChild?: boolean
}

interface DropdownMenuContentProps {
  children: ReactNode
  align?: "start" | "center" | "end"
  className?: string
}

interface DropdownMenuItemProps {
  children: ReactNode
  onSelect?: () => void
  asChild?: boolean
}

interface DropdownMenuLabelProps {
  children: ReactNode
}

interface DropdownMenuSeparatorProps {}

export const DropdownMenu = ({ children, trigger, align = "start", className = "" }: DropdownMenuProps) => {
  const [isOpen, setIsOpen] = useState(false)
  const dropdownRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false)
      }
    }

    document.addEventListener("mousedown", handleClickOutside)
    return () => document.removeEventListener("mousedown", handleClickOutside)
  }, [])

  return (
    <div ref={dropdownRef} className={`relative ${className}`}>
      <div onClick={() => setIsOpen(!isOpen)}>{trigger}</div>
      {isOpen && (
        <div className={`absolute top-full mt-1 z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md animate-in data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ${
          align === "start" ? "left-0" : align === "center" ? "left-1/2 transform -translate-x-1/2" : "right-0"
        }`}>
          {children}
        </div>
      )}
    </div>
  )
}

export const DropdownMenuTrigger = ({ children, asChild }: DropdownMenuTriggerProps) => {
  return <>{children}</>
}

export const DropdownMenuContent = ({ children, align = "start", className = "" }: DropdownMenuContentProps) => {
  return <div className={className}>{children}</div>
}

export const DropdownMenuItem = ({ children, onSelect }: DropdownMenuItemProps) => {
  return (
    <div
      className="relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50"
      onClick={onSelect}
    >
      {children}
    </div>
  )
}

export const DropdownMenuLabel = ({ children }: DropdownMenuLabelProps) => {
  return (
    <div className="px-2 py-1.5 text-sm font-semibold">
      {children}
    </div>
  )
}

export const DropdownMenuSeparator = ({}: DropdownMenuSeparatorProps) => {
  return <div className="border-t border-border my-1" />
} 