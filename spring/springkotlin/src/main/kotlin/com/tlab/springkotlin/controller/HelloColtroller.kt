package com.tlab.springkotlin.controller

import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RequestMethod
import org.springframework.web.bind.annotation.RestController

@RestController
class HelloColtroller {

    @RequestMapping("/", method = [RequestMethod.GET])
    fun index(): String{
        return "Hello Spring"
    }
}